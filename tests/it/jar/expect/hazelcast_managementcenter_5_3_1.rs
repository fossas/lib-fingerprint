use fingerprint::Kind;

use crate::jar::DiscoveredJar;

use super::combined;

pub fn list() -> Vec<DiscoveredJar> {
    vec![
        DiscoveredJar::new(
            "opt/hazelcast/management-center/hazelcast-management-center-5.3.1.jar",
            combined! {
                Kind::JarRawV1 => "e8905ad74a9526d981986ca7759a503db9f3f2739d984a1bef1d7e212fcf416d",
                Kind::JarClassV1 => "ad8508d98430fc9ab8bafb04163b8c4f8ee896d48724f3d17f834ddc22cc88af",
                Kind::RawSha256 => "5f2db487a33eeb0e3daeceb096952a00a0936b5fd9446704368578553053c7e4",
                Kind::JarMavenCentralV1 => "9d281779255a4c2b4a9a034236d7e48f938db2a7",
            },
        ),
        DiscoveredJar::new(
            "opt/hazelcast/management-center/hazelcast-management-center-5.3.1.jar",
            combined! {
                Kind::JarRawV1 => "e8905ad74a9526d981986ca7759a503db9f3f2739d984a1bef1d7e212fcf416d",
                Kind::JarClassV1 => "ad8508d98430fc9ab8bafb04163b8c4f8ee896d48724f3d17f834ddc22cc88af",
                Kind::RawSha256 => "5f2db487a33eeb0e3daeceb096952a00a0936b5fd9446704368578553053c7e4",
                Kind::JarMavenCentralV1 => "9d281779255a4c2b4a9a034236d7e48f938db2a7",
            },
        ),
        DiscoveredJar::new(
            "usr/lib/jvm/java-17-openjdk-17.0.7.0.7-3.el8.x86_64/lib/jrt-fs.jar",
            combined! {
                Kind::JarClassV1 => "1b9d4816dad40a7922d991be24f0f1bc4bffaf271ab45b9ba32f123a932dd807",
                Kind::JarRawV1 => "3788fc905d4f2aaf615252c8fcbfc1ec8fdc25b4538f194603b79d9d571ab7d4",
                Kind::RawSha256 => "f05c638cc96a6e119aef7044764c189a089ed9abfbd765efe33774bc94afa0d5",
                Kind::JarMavenCentralV1 => "a714642363ddfcc36d5b7e535c2f2d215e9eb648",
            },
        ),
    ]
}
