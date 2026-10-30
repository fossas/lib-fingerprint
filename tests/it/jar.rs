//! Tests for JAR fingerprinting.
//!
//! Highly recommended to run these tests with `--release`:
//! ```ignore
//! cargo nextest run --release
//! ```

use std::{
    collections::HashSet,
    fs::File,
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use alphanumeric_sort::compare_str;
use fingerprint::{Combined, Content, Fingerprint, Kind};
use itertools::Itertools;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tap::Pipe as _;
use tar::Entry;
use tracing::{debug, info, info_span};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

pub mod expect;

/// Like [`pretty_assertions::assert_eq`], meant for two vectors of [`DiscoveredJar`].
///
/// 1. Sorts the two vectors for stable comparison.
/// 2. Special cases the [`fingerprint::Combined`] inside for map-style comparison.
macro_rules! assert_eq_jars {
    ($expected:expr, $received:expr) => {{
        let mut expected = $expected.into_iter().map(CmpJar::from).collect::<Vec<_>>();
        let mut received = $received.into_iter().map(CmpJar::from).collect::<Vec<_>>();
        expected.sort_unstable();
        received.sort_unstable();
        pretty_assertions::assert_eq!(expected, received);
    }};
}

#[test_log::test]
#[cfg_attr(not(feature = "docker-tests"), ignore = "docker tests not enabled")]
fn docker_elasticsearch_7_17_17() {
    let jars = jars_in_container("elasticsearch:7.17.17");
    assert_eq_jars!(expect::elasticsearch_7_17_17::list(), jars);
}
#[test_log::test]
#[cfg_attr(not(feature = "docker-tests"), ignore = "docker tests not enabled")]
fn docker_bitnami_elasticsearch_7_17_17_debian_11_r4() {
    let jars = jars_in_container("bitnami/elasticsearch:7.17.7-debian-11-r4");
    assert_eq_jars!(
        expect::bitnami_elasticsearch_7_17_17_debian_11_r4::list(),
        jars
    );
}
#[test_log::test]
#[cfg_attr(not(feature = "docker-tests"), ignore = "docker tests not enabled")]
fn docker_hazelcast_managementcenter_5_3_1() {
    let jars = jars_in_container("hazelcast/management-center:5.3.1");
    assert_eq_jars!(expect::hazelcast_managementcenter_5_3_1::list(), jars);
}

/// Extracts the container (saved via `docker save`) and finds JAR files inside any layer.
/// For each one found, fingerprints it and reports all those fingerprints along with their
/// layer and path.
#[track_caller]
#[tracing::instrument]
fn jars_in_container(container: &str) -> Vec<DiscoveredJar> {
    // Rust libraries exist for interacting with docker directly,
    // but since this is only for testing I think it's easier to interact via the shell.
    let sh = xshell::Shell::new().expect("create shell");

    // Force the `linux/amd64` platform when pulling, since that's what CI runs in.
    // This way local tests work even on M-series macOS, and CI works even though it's linux.
    info!("pulling container");
    xshell::cmd!(sh, "docker pull --platform linux/amd64 {container}")
        .run()
        .expect("pull container");

    let dir = tempfile::tempdir().expect("create temp directory");
    let container_file = lazy_regex::regex_replace_all!(r"[^A-Za-z0-9_]", container, "_");
    let destination = dir.path().join(container_file.as_ref());
    info!(?destination, "saving container to disk");
    xshell::cmd!(sh, "docker save {container} -o {destination}")
        .run()
        .expect("save container");

    if !destination.exists() {
        panic!("container does not exist at {destination:?}");
    }

    // Visit each layer and fingerprint the JARs within.
    info!("inspecting container");
    let layers = list_container_layers(&destination);
    let archive = File::open(&destination).expect("open file");
    let mut discoveries = Vec::new();
    for entry in unpack(archive).entries().expect("list entries") {
        let entry = entry.expect("read entry");
        let path = entry.path().expect("read path");
        if !layers.contains(path.as_ref()) {
            debug!(?path, "skipped: not a layer file");
            continue;
        }

        let layer = path.to_path_buf();
        let layer_discoveries = jars_in_layer(layer, entry);
        discoveries.extend(layer_discoveries);
    }

    discoveries
}

#[track_caller]
#[tracing::instrument(skip(entry))]
fn jars_in_layer(layer: PathBuf, entry: Entry<impl Read>) -> Vec<DiscoveredJar> {
    let mut discoveries = Vec::new();
    for entry in unpack(entry).entries().expect("list entries in layer") {
        let entry = entry.expect("read entry");
        let path = entry.path().expect("read path");
        if !path.to_string_lossy().ends_with(".jar") {
            debug!(?path, "skipped: not a jar file");
            continue;
        }

        let path = path.to_path_buf();
        info_span!("jar", ?path).in_scope(|| {
            debug!("fingerprinting");
            let entry = buffer(entry);
            discoveries.push(DiscoveredJar {
                fingerprint: Combined::from_buffer(entry).expect("fingerprint"),
                path,
            });
        });
    }

    discoveries
}

#[track_caller]
#[tracing::instrument(skip(container), ret)]
fn list_container_layers(container: &Path) -> HashSet<PathBuf> {
    let archive = File::open(container).expect("open file");
    let mut layers = HashSet::new();

    for entry in unpack(archive).entries().expect("list entries") {
        let entry = entry.expect("read entry");
        let path = entry.path().expect("read path");
        if !path.ends_with("manifest.json") {
            debug!(?path, "skipped: not a manifest file");
            continue;
        }

        info!(?path, "extracting manifests for image");
        let manifests: Vec<OciManifest> = serde_json::from_reader(entry).expect("read manifest");
        for manifest in manifests {
            layers.extend(manifest.layers);
        }

        // There's only one manifest file.
        break;
    }

    layers
}

#[track_caller]
#[tracing::instrument(skip(reader))]
fn unpack<R: Read>(reader: R) -> tar::Archive<R> {
    tar::Archive::new(reader)
}

#[track_caller]
#[tracing::instrument(skip(reader))]
fn buffer(mut reader: impl Read) -> Vec<u8> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("buffer bytes");
    buf
}

#[derive(Debug, PartialEq, Eq)]
struct DiscoveredJar {
    path: PathBuf,
    fingerprint: Combined,
}

impl DiscoveredJar {
    fn new(path: impl Into<PathBuf>, fp: impl Into<Combined>) -> Self {
        Self {
            path: path.into(),
            fingerprint: fp.into(),
        }
    }
}

/// Special case of [`DiscoveredJar`] for comparing
/// in a platform-independent manner.
struct CmpJar {
    path: String,
    fingerprint: Vec<CmpFingerprint>,
    hash: String,
}

impl std::fmt::Debug for CmpJar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CmpJar")
            .field("path", &self.path)
            .field("fingerprint", &self.fingerprint)
            .finish()
    }
}

impl std::cmp::PartialEq for CmpJar {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.hash == other.hash
    }
}

impl std::cmp::Eq for CmpJar {}

impl From<DiscoveredJar> for CmpJar {
    fn from(jar: DiscoveredJar) -> Self {
        let fingerprint = jar
            .fingerprint
            .into_inner()
            .into_iter()
            .map(CmpFingerprint::from)
            .sorted()
            .collect_vec();
        let hash = hash_kind_content(&fingerprint).pipe(hex::encode);

        Self {
            path: jar.path.to_string_lossy().to_string().replace('\\', "/"),
            fingerprint,
            hash,
        }
    }
}

impl std::cmp::Ord for CmpJar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match alphanumeric_sort::compare_str(&self.path, &other.path) {
            std::cmp::Ordering::Equal => alphanumeric_sort::compare_str(&self.hash, &other.hash),
            ord => ord,
        }
    }
}

impl std::cmp::PartialOrd for CmpJar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Special case of fingerprint for comparing in tests.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct CmpFingerprint {
    kind: Kind,
    content: Content,
}

impl std::fmt::Debug for CmpFingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{:?} => "{:?}""#, self.kind, self.content)
    }
}

impl From<(Kind, Content)> for CmpFingerprint {
    fn from((kind, content): (Kind, Content)) -> Self {
        Self { kind, content }
    }
}

/// Generate a hash of the map:
/// - Entries are sorted by [`Kind`].
/// - [`Content`]s are then hashed in sort order.
///
/// The intention isn't to make a _stable_ hash or anything,
/// it's literally just to support ordering [`CmpJar`]
/// in cases where the same path is read from multiple layers in an image.
fn hash_kind_content(cmp: &[CmpFingerprint]) -> Vec<u8> {
    cmp.iter()
        .sorted_by(|a, b| a.cmp(b))
        .fold(Sha256::new(), |mut hasher, cmp| {
            hasher.update(cmp.content.as_bytes());
            hasher
        })
        .pipe(|hasher| hasher.finalize().as_slice().to_vec())
}

/// Not a full manifest, just the part we care about.
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct OciManifest {
    #[serde(rename = "Layers")]
    layers: Vec<PathBuf>,
}

/// Builds a ZIP/JAR in memory with the given `(name, payload)` entries using
/// `Stored` compression and **no** UTF-8 global flag (the writer default), so each
/// name is stored as raw bytes with the language-encoding bit unset.
fn build_jar(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    for (name, payload) in entries {
        zip.start_file(*name, options)
            .expect("add entry to synthetic jar");
        zip.write_all(payload).expect("write synthetic entry");
    }
    zip.finish().expect("finish synthetic jar").into_inner()
}

/// The JAR fingerprint hashes each entry's *content* in sorted-filename order, so a
/// filename *decode* change only leaks out when it reorders entries. zip 9 decodes
/// valid-UTF-8 bytes in unflagged entries as UTF-8, whereas zip 3 decoded those same
/// bytes as CP437 — which reorders the pair below. This pins the zip-9 (UTF-8) behavior
/// so a dependency downgrade or decode regression can't silently change fingerprints.
///
///   `\u{c3}\u{a9}.class` -> UTF-8 `é.class`  / CP437 `├\u{2310}.class`
///   `\u{c3}\u{ac}.class` -> UTF-8 `ì.class`  / CP437 `├\u{00bc}.class`
///   UTF-8 sort: é < ì ;  CP437 sort: ├\u{00bc} < ├\u{2310}  (reversed)
#[test]
fn jar_class_filename_decode_is_utf8_not_cp437() {
    let archive = build_jar(&[
        ("\u{00e9}.class", b"data1"), // written first
        ("\u{00ec}.class", b"data2"), // written second
    ]);

    let got = Fingerprint::from_buffer(Kind::JarClassV1, &archive)
        .expect("valid archive")
        .expect("two class entries");

    // Independent re-derivation of the fingerprint using zip-9 (UTF-8) names: each
    // entry's content is Sha256(payload), and combine() folds those in filename order.
    let mut entries: Vec<(&str, Vec<u8>)> = vec![
        ("\u{00e9}.class", Sha256::digest(b"data1").to_vec()),
        ("\u{00ec}.class", Sha256::digest(b"data2").to_vec()),
    ];
    entries.sort_by(|a, b| compare_str(a.0, b.0));
    let mut combined = Sha256::new();
    for (_, digest) in &entries {
        combined.update(digest);
    }
    let expected = Content::from_digest(combined);
    assert_eq!(
        got.content(),
        &expected,
        "names must decode as UTF-8 (zip 9)"
    );

    // Prove the test is actually sensitive: CP437 decoding reorders these entries,
    // which must produce a different fingerprint.
    let mut cp437: Vec<(&str, Vec<u8>)> = vec![
        ("\u{251c}\u{2310}.class", Sha256::digest(b"data1").to_vec()),
        ("\u{251c}\u{00bc}.class", Sha256::digest(b"data2").to_vec()),
    ];
    cp437.sort_by(|a, b| compare_str(a.0, b.0));
    let mut combined_cp437 = Sha256::new();
    for (_, digest) in &cp437 {
        combined_cp437.update(digest);
    }
    assert_ne!(
        *got.content(),
        Content::from_digest(combined_cp437),
        "CP437 decode must reorder entries and change the fingerprint"
    );
}
