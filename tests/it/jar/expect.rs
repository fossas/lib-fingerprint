//! The lists of JAR files to expect inside of these containers get _quite_ long.
//! These modules exist to separate them out from test logic.
//!
//! The lists of JAR files here have been spot checked but not exhaustively reviewed
//! by hand (there's just too many without any real indication of a problem).

use fingerprint::Content;
use tap::Pipe as _;

pub(super) mod bitnami_elasticsearch_7_17_17_debian_11_r4;
pub(super) mod elasticsearch_7_17_17;
pub(super) mod hazelcast_managementcenter_5_3_1;

/// Create a new instance of [`fingerprint::Combined`] with the provided literal.
/// The provided values are decoded as hex strings for the fingerprints.
///
/// ```
/// combined! {
///     Kind::JarRawV1 => "fda6923b9f32df9a02ad83699ceefbdafb408abfa3c8c917592653f53028db4c",
///     Kind::JarClassV1 => "c21995af9737437c4111a6054c8bc2a6d7ac00c90b788b0b6d7b2489ec45973c",
///     Kind::RawSha256 => "8a8da0f4cbac56bb4c79c49ffae1e30184a46d4495faf701b9ceadaf2a2684e3",
/// }
/// ```
macro_rules! combined {
    {$($key:expr => $value:expr),* $(,)?} => {
        fingerprint::Combined::from(maplit::hashmap!{
            $(
                $key => $crate::jar::expect::content_hex($value),
            )*
        })
    };
}

/// Create a new [`Content`] instance from a hex-encoded string.
pub fn content_hex(encoded: impl AsRef<[u8]>) -> Content {
    hex::decode(encoded)
        .expect("must decode")
        .pipe(Content::new)
}

pub(crate) use combined;
