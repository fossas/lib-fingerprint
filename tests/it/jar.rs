use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use fingerprint::{Combined, Content, Kind};
use serde::Deserialize;
use tar::Entry;
use tracing::{debug, info_span, trace};

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
fn elasticsearch_7_17_17() {
    let jars = jars_in_container("testdata/images/elasticsearch_7_17_17.tar");
    assert_eq_jars!(expect::elasticsearch_7_17_17::list(), jars);
}
#[test_log::test]
fn bitnami_elasticsearch_7_17_17_debian_11_r4() {
    let jars = jars_in_container("testdata/images/bitnami_elasticsearch_7_17_17_debian_11_r4.tar");
    assert_eq_jars!(
        expect::bitnami_elasticsearch_7_17_17_debian_11_r4::list(),
        jars
    );
}
#[test_log::test]
fn hazelcast_managementcenter_5_3_1() {
    let jars = jars_in_container("testdata/images/hazelcast_managementcenter_5_3_1.tar");
    assert_eq_jars!(expect::hazelcast_managementcenter_5_3_1::list(), jars);
}

/// Extracts the container (saved via `docker save`) and finds JAR files inside any layer.
/// For each one found, fingerprints it and reports all those fingerprints along with their
/// layer and path.
///
/// The path to the container should be from the crate root (the directory containing `Cargo.toml`).
#[track_caller]
#[tracing::instrument]
fn jars_in_container(container: impl AsRef<Path> + std::fmt::Debug) -> Vec<DiscoveredJar> {
    let container = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(container);
    debug!("inspecting container");

    // Visit each layer and fingerprint the JARs within.
    let layers = list_container_layers(&container);
    let archive = File::open(&container).expect("open file");
    let mut discoveries = Vec::new();
    for entry in unpack(archive).entries().expect("list entries") {
        let entry = entry.expect("read entry");
        let path = entry.path().expect("read path");
        if !layers.contains(path.as_ref()) {
            trace!(?path, "skipped: not a layer file");
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
            trace!(?path, "skipped: not a jar file");
            continue;
        }

        let path = path.to_path_buf();
        info_span!("jar", ?path).in_scope(|| {
            debug!("fingerprinting");
            let entry = buffer(entry);
            discoveries.push(DiscoveredJar {
                fingerprint: Combined::from_buffer(entry).expect("fingerprint"),
                layer: layer.clone(),
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
            trace!(?path, "skipped: not a manifest file");
            continue;
        }

        debug!(?path, "extracting manifests for image");
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
    layer: PathBuf,
    path: PathBuf,
    fingerprint: Combined,
}

impl DiscoveredJar {
    fn new(layer: impl Into<PathBuf>, path: impl Into<PathBuf>, fp: impl Into<Combined>) -> Self {
        Self {
            layer: layer.into(),
            path: path.into(),
            fingerprint: fp.into(),
        }
    }
}

/// Special case of [`DiscoveredJar`] for comparing
/// in a platform-independent manner.
#[derive(Debug, PartialEq, Eq)]
struct CmpJar {
    layer: String,
    path: String,
    fingerprint: HashMap<Kind, Content>,
}

impl From<DiscoveredJar> for CmpJar {
    fn from(jar: DiscoveredJar) -> Self {
        let into_string = |s: PathBuf| s.to_string_lossy().to_string();
        Self {
            layer: into_string(jar.layer).replace('\\', "/"),
            path: into_string(jar.path).replace('\\', "/"),
            fingerprint: jar.fingerprint.into_inner(),
        }
    }
}

impl std::cmp::Ord for CmpJar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match alphanumeric_sort::compare_str(&self.layer, &other.layer) {
            std::cmp::Ordering::Equal => alphanumeric_sort::compare_str(&self.path, &other.path),
            ord => ord,
        }
    }
}

impl std::cmp::PartialOrd for CmpJar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Not a full manifest, just the part we care about.
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct OciManifest {
    #[serde(rename = "Layers")]
    layers: Vec<PathBuf>,
}
