use fingerprint::Kind;

use crate::jar::DiscoveredJar;

use super::combined;

pub fn list() -> Vec<DiscoveredJar> {
    vec![
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/bin/elasticsearch-sql-cli-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "1274453883544be7c0286534a863bfa338be13bf17534019d1b0655bfc6e7be1",
                Kind::RawSha256 => "657a3c1414aff4e9d7d170a11cea8cbfe9b6d5cae0dd630a3a83126a7d701a42",
                Kind::JarClassV1 => "8e6d9e674a30845f386f4a4bff084aac6b2c78c1082dc11657fc14541eb45e06",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/HdrHistogram-2.1.9.jar",
            combined! {
                Kind::JarClassV1 => "36369d225d43ba9449746c5151bc56030d741ec5069a79bba516383a78e0d93d",
                Kind::JarRawV1 => "9158db0485c6b3455e379e563e925ce03f4b5d81e62bf10ece305b8d14c30e0e",
                Kind::RawSha256 => "95d40913be28dfd439cefea9170c40898ea84f11f25e6ff8de50339b8a7b5e3e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "081ff7b3a5b7019048613ac1448ed74b3517ff9cfe5a54284372cc50c8b6fdba",
                Kind::JarClassV1 => "ca7f10fcac312f2952f55ea189aac82316733997c0a940c0de4316c66ccf345b",
                Kind::RawSha256 => "3073c8a5d8c43e21507f6bdc3f099f7393b602d8474435587aaa0ef52d96c480",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-cli-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "fd3fde60105ec3a1a32bbadde147fed507fc1340c3e28728c3eeffa8060ba406",
                Kind::JarRawV1 => "de4d7bb4ec5b2dd62d10755f8638147e2ff95c9428666de4b3718217f2d9f19d",
                Kind::JarClassV1 => "62eb697375fa4ab5cab180d212a46e62cf710f7a0832f99bcc9831b88dd56259",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-core-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "3503572b285f6450b1a23021a573023f15e3626d7cfb42fd52b1dea44beacf16",
                Kind::JarClassV1 => "c0b6a3f3e8a1f005b75c64c8725add97837a07669c56124bc17469e6f4fdaae3",
                Kind::RawSha256 => "df7328b688eb01fe5556235e80eb75c14ad0c0e1d23a9f5ff721735909094f8c",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-geo-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "0a7728cd669175fad43bf7387d46c9bb89245ede9918c334975dd8c558bd2841",
                Kind::JarRawV1 => "c5a03d4aadf5ea2c1cd70ca1e85e8fe1435140d7aa36e55d6e10111674008826",
                Kind::JarClassV1 => "331bcd766ecd88403efebec0cb4d2c130ca4d06e16f027c5a6be877511abe79a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-launchers-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "c8fe6c7dfabba48bebf76df7bb3ef4ddee7573353ffe05fda513fdeed47b03ad",
                Kind::JarRawV1 => "207c4f63dac238d2179a56ef51ed7eca8534a019653006cd4b5bf1aa0d3df20a",
                Kind::JarClassV1 => "3861758cd4c43a8391310e0a5caac44acf8730376eefda433988dfc41bcd4843",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-log4j-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "fbc4adb18d1d3b49b639491d51d333fdb43395d0fdbb1d7ecbc960f981f3a005",
                Kind::JarRawV1 => "5452acf8354a640b29a1625a2de876891e5677914bd9d04842e17f10a03caa73",
                Kind::RawSha256 => "a5b68ae4a9634e0e1f351b6eb5bd10f9b936b7b6997ebe20ff8d2a94550b218f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-lz4-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "d0c726abfe6cc4c604440265f24c1574a843bcbd3dcefe8210a232c0ab23fecb",
                Kind::JarClassV1 => "e5044e0168e40068300126a309a55277b97f47964c0d737e77961311fa0f15f1",
                Kind::RawSha256 => "b30307c3a455eefbfb126a5eb344b29a13655f889c5d1fb2bf66ea12de603cdb",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-plugin-classloader-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "d57f4ebb3e6a9f266936e470e5af037de335074881e5c95d94b4b5731961dece",
                Kind::JarRawV1 => "4a329fcac84f96b77f9a724e7845b29de7016bae83169ba0e336eb1ed832090a",
                Kind::RawSha256 => "d276e691a7396fe76f6cd4eaf2054a67ff8efabb9c815074f2d6959aade7aa9a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-secure-sm-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "53fe0ab4049791fee84c9f810801b058fcc49637ba3446a5ee342e9e168a9063",
                Kind::JarRawV1 => "aa1a0e9b28749ec2d2f534937ef581fb7d7b4aec493efb9269268f25d29b592b",
                Kind::JarClassV1 => "4cbba4d500da66a970737a5ce3acca786c74a2454ee91cfa17fce740da087585",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/elasticsearch-x-content-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "f3cd44667ef1ab92d13c41c0b4ed1a47bbb399467c34ad2977d62b0f11559678",
                Kind::RawSha256 => "25f03c399cab7d3c31421aa0bfcdea8a10b71125e36d99b31fb0ae79e64a8e4a",
                Kind::JarClassV1 => "6cd4e8b535d4de6ec0206903260d275ddfe165d3d79c2542858d882ca742cf46",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/hppc-0.8.1.jar",
            combined! {
                Kind::JarClassV1 => "ebb54d52e93451f08dfd84367e8702073c52048a5b124653ee2208cd8e3b263f",
                Kind::RawSha256 => "f540703478636d88f699f4666242e6fc9175a996c08ddceaf02106517b970406",
                Kind::JarRawV1 => "97acc4f0f2b9350ea1771ccedc9c22b9220730160e8e924d4b59339898d1d3a5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/jackson-core-2.10.4.jar",
            combined! {
                Kind::JarRawV1 => "d83dc950a67a0c542ba3dc415d4d25bf26b7f09483f0c59d065ddf2c6fb5cf50",
                Kind::RawSha256 => "564f6e5706096179537114299e6d7492d2c38da182df8d7834a4c9141b078ef3",
                Kind::JarClassV1 => "21cc119fffa6438a55312467a74fe489e336c65a6a9f83e94265bbba8f1f188b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/jackson-dataformat-cbor-2.10.4.jar",
            combined! {
                Kind::RawSha256 => "5e15ea426db254c2e98454165b2d9f8dffc4be1f2da4aa18eedf7d7c076cdb59",
                Kind::JarRawV1 => "c9de40514175825a201aed730974306eeeb274e7aa91535fb56a638fdc7e127a",
                Kind::JarClassV1 => "bf72f2573e22b3b7749fb25ba2a0aea1bf8d9782fadcef8505e02834741bd3e3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/jackson-dataformat-smile-2.10.4.jar",
            combined! {
                Kind::JarRawV1 => "928865d27717481e6cf02e2011791c60341b3c9fbac64d02f45242d46aee4c6d",
                Kind::JarClassV1 => "4d3022e1cd6abc042a3eb0aeeb3f81fba40189c9ddbd803c45f2b4c6485b8086",
                Kind::RawSha256 => "21cb36dbe1eb4782edc35d6eb5da82720f92fbfa3c440f6680e50d8ac65750db",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/jackson-dataformat-yaml-2.10.4.jar",
            combined! {
                Kind::JarClassV1 => "61f0ba7cf90480fa9f05f036b6322d26fc74de688950118fdb0fc1cfb6d0f5c4",
                Kind::RawSha256 => "47bf4e8ad64e87def3f4f9ab73c7ed5ca9b51c53cb203b622c29b9640825512c",
                Kind::JarRawV1 => "82ed9d506ecd2dc012b2fdc8cb16462849f4fdf16ee89a9d601e1357209e02a3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/java-version-checker-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "ade2a19e45fce23bfd9abcc82f0610b8d4da36b376bdc3481ddb8af2d9201ed7",
                Kind::JarRawV1 => "2cff2d1a95af854a34b3d58cd2b181c0ec2a1c2d7a3708f86b27e05608ec419c",
                Kind::JarClassV1 => "1b661649431bf66a202b3f5d2bae0eb0fcb3f8fa087928351ca8a97f2aba4545",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/jna-5.10.0.jar",
            combined! {
                Kind::JarRawV1 => "4d9fa8b2674038b2216d11345637883005c3f3b36c6df2caf98cb4b4505531cd",
                Kind::RawSha256 => "e335c10679f743207d822c5f7948e930319835492575a9dba6b94f8a3b96fcc8",
                Kind::JarClassV1 => "3182c4cad937014c678e251f59e2c5807ec98ddf45dbceda25fecf2e2f698d42",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/joda-time-2.10.10.jar",
            combined! {
                Kind::JarRawV1 => "8cdcb8920fe090b6c05e6857c98fe76d64e77952277b8ea48b921e0cd20249a5",
                Kind::RawSha256 => "dd8e7c92185a678d1b7b933f31209b6203c8ffa91e9880475a1be0346b9617e3",
                Kind::JarClassV1 => "2ad59c72014c59354a69407845e1af9f487a61988e1abc471f57191fab868885",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/jopt-simple-5.0.2.jar",
            combined! {
                Kind::RawSha256 => "457877c79e038f390557db5f8e92c4436fb4f4b3ba63f28bc228500fee080193",
                Kind::JarClassV1 => "4512e3d8881b866ff8302e154c2a53f25353c984f14499db71af35850d527275",
                Kind::JarRawV1 => "cb9c567ee8cb2409fef0287fc4418ff3d7bf54d1c7179ecdd58e2702dca4118a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/log4j-api-2.17.1.jar",
            combined! {
                Kind::RawSha256 => "b0d8a4c8ab4fb8b1888d0095822703b0e6d4793c419550203da9e69196161de4",
                Kind::JarRawV1 => "85afc83a396aadb819b3dc39886ede6356677811b4f74a3e1a1e77cf1d631792",
                Kind::JarClassV1 => "e456420a1a09c1e3ed22328c067c9d43d1ab969d06fa7529ed92e7df894764e6",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-analyzers-common-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "d4c73f31b1267f22eccd55f117a0ad86e705bb5284b796fd9408aedb48591d29",
                Kind::JarClassV1 => "a3d58f1366ea2d755db2e0f0ce89fea089b363765076f806ae3f0892a530438a",
                Kind::RawSha256 => "1cdcc5a2d9cf4ffaf12fbf24bc2a18f2469cd295b60470ae8b97d1aa85dbad6f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-backward-codecs-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "38e72688eef81efffb9c5ea68918f4d1adb2eb0de64ce6a8222abee036eb63cf",
                Kind::JarRawV1 => "78bd321ca006136c777f94841703de765592a3daed70b42c14de215b28fb428f",
                Kind::JarClassV1 => "f5cb54cb9f9291c9719b92335ab46a082fb4ec33db736260cc7aeb33f710b4a0",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-core-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "16e949a443e6c2ac86258f6754539bd518fca2ab2615147b82d8b3e6b093181c",
                Kind::JarClassV1 => "9ebcb4f23a1c7198faac110ece5d774891cf2fa173c936671830466ba2c434b2",
                Kind::RawSha256 => "78a61d0b843c1cf1fe5be380a4d3a4c1602d3fbba4ca1185da8797c9bb115483",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-grouping-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "c87fd0c311b8c712088536faf1dc2254a01a0d7c73e9ed565caa08ddc67e2500",
                Kind::JarRawV1 => "f09a0215c8b5812bf0b003b8b9989216afb9af56e1869eb636cb65ec803b1fe5",
                Kind::JarClassV1 => "59e390cb42b732ffe97f5d15c79e799d28ae30972db0a3ab69e2d740fc7274d5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-highlighter-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "f09a910b20cfc4a42aeb62ba99875670357f8274ebd375a378a96f95ab86d5b3",
                Kind::JarClassV1 => "d2f0c25a7c0c08b5e36bd76a0796fd61344bc240351b0bddd0dc7bfa6ab7988d",
                Kind::RawSha256 => "c8e92e01b7443b2fd1698ac7b260b632197175143e13f0364b6f7258b9447307",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-join-8.11.1.jar",
            combined! {
                Kind::JarClassV1 => "672e51858da6844b76e1ae711751741fa0879527c872ae10fbdf83b496bd4267",
                Kind::JarRawV1 => "2d04ce13216b56953b7bd3326e4aa072697b6b1e10a5601cca903dbed56a187c",
                Kind::RawSha256 => "20a1912bfa2283519ee9c4b28eb559c0757187880e6783e04779ade44eb16757",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-memory-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "b948478fe2e8e7f94fa7f533a14a526720701fe98627214c93d924faa4be78de",
                Kind::JarClassV1 => "1edbe9bc4dc89baefa6a173fd5183475ae6c521e448f1aa2c707ff258f4f1ac1",
                Kind::JarRawV1 => "d12dd9296186e6b5a9c43ee95c7028102f543e0e9eaab9385419e759dc6611c8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-misc-8.11.1.jar",
            combined! {
                Kind::JarClassV1 => "53e462f256988941598804f5df5b280ccf0698a4a0167107089feb02ba627813",
                Kind::RawSha256 => "4a1102663cba35676616038b516d487ee3db9802fade99649e14f09c8b412cf3",
                Kind::JarRawV1 => "e7d891cd5f488afbd5b5b81ce42cea062ba1e832f55c89441797e1bcfb2a695f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-queries-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "a74dd3980e3a94e4194ad9da79cbb91b554e383f53acc2a0885cc62186e0a844",
                Kind::RawSha256 => "11fb2e90da5b4e6a6c26120bb80a2937a20a585d32236ed7c277048ba65f07ca",
                Kind::JarClassV1 => "26f69dc1ae4fc85526033d5064776bab323c5c1873f2658de302281b838414aa",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-queryparser-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "f79f96d3a9ccf7fc9c438178f1600c1ebfe09fe29dae04068a9ad2fb83db76c0",
                Kind::JarClassV1 => "d77ae0518068d04fd76c85e01ddce9528bbd55ef233272e9073749eb969a7fb6",
                Kind::RawSha256 => "23abf022a19e609fe3ca421ab6b6868a3250974d31c5b92f9879d97c127a77b8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-sandbox-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "28bee2711947cf3a9957f3f77132ce37457894c1fb468b0a20e9a95788b11c87",
                Kind::JarClassV1 => "9d88cfebbc1e31cf6e9df3b5355c6fca570e9883a66d259ae06ca015d8cc8763",
                Kind::JarRawV1 => "849df1f98ae16c6ebf0379c67dfe326a2ceee42bbc7db0da4ae3c9856d6c4530",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-spatial3d-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "fb98a09f8da5e912b2c5dcfe6353373a3309cdbf11e64031462cd9d19bd89256",
                Kind::JarClassV1 => "ef347f6f1e3107992a035ce434d9db09b7a60ec130d7035bc00d0c783e1c313f",
                Kind::JarRawV1 => "c907dbca85ff43726c831dcd6773486dd6f6b29f5d53f0babe9c193053a2c7cd",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lucene-suggest-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "e328709455e56a1b41c270e69d77ee98c116d27fc5dca6cd3972c45dadb3d23a",
                Kind::JarRawV1 => "b678df5cf98757e9ed6af8143f5b7b2448d94f907f54d48520817b899092a787",
                Kind::JarClassV1 => "b75b3194e53db4a56b7b847a006eb08631b1ca6e7bb8019ba1836da5d511d17b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/lz4-java-1.8.0.jar",
            combined! {
                Kind::JarClassV1 => "e5d613383bc81df7584c0cecc045daeba675d80c17874ebafe1760fdd5bb37f2",
                Kind::RawSha256 => "d74a3334fb35195009b338a951f918203d6bbca3d1d359033dc33edd1cadc9ef",
                Kind::JarRawV1 => "c5d86569d7e758f1cca5d639964845b7d5a0550e6165a184145aca3bbbb5f66a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/snakeyaml-1.33.jar",
            combined! {
                Kind::RawSha256 => "11ff459788f0a2d781f56a4a86d7e69202cebacd0273d5269c4ae9f02f3fd8f0",
                Kind::JarRawV1 => "dcee9e292c05e428c43951d237db7ab32089b4c6d4c51a90e8eb226867fe30d6",
                Kind::JarClassV1 => "a0a7b22ba470b11ab71ae8bd0be0ef8816355a9a9e8fb2a427991dfd513ee360",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/t-digest-3.2.jar",
            combined! {
                Kind::RawSha256 => "03db291a8887b474f90db67bfb1f92d084e990150037e231babbb374ee11d7c3",
                Kind::JarClassV1 => "db500040a066ab5003b524b31dc0afe4c78894ec91be799fe815c11cb6ee1dda",
                Kind::JarRawV1 => "29ceca9b46833a1baa93b205476f63f196d53b2349e655c8df6cab2df455b4fe",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/tools/geoip-cli/elasticsearch-geoip-cli-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "bee4dc9a79c385e4148a536e05bb3658725dbcc9308c8574725ba8b0c612ce14",
                Kind::RawSha256 => "25f4e69c5353fa789f0441f3e4e2d6ac19586dbbb01943ddf5e7edf743037393",
                Kind::JarRawV1 => "27128e15036522e8365f5edc0f711793c3fb56834f63b86915e9f127681749f2",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/tools/keystore-cli/keystore-cli-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "7379c171a71e4a0aebea343238fd02e79c1d40b53e9a4e6bb981addbf1f44778",
                Kind::JarRawV1 => "642a4b5b403b0b584c82541d01e29cef1afa5a79180c48d8e230da66881caf7b",
                Kind::JarClassV1 => "b01f752b5fa626e509805aef7e1e5a7ccf9b86a1769d3398b9b2755d6de694dc",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/tools/plugin-cli/bc-fips-1.0.2.jar",
            combined! {
                Kind::RawSha256 => "b4340d7a9cc0d3664d6c560e2fcee9c7da6e6ae314855923c758fa32fff5b94e",
                Kind::JarClassV1 => "1bf22d29dbf6e2ce4fd7efda27960416f842c60d81fbf526f3ffdd15ba7cb80f",
                Kind::JarRawV1 => "e2c0b8ac63913dafd3d07f3aa4e11c2b5c5fe37581fe921d19ac8cbe21f0ce87",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/tools/plugin-cli/bcpg-fips-1.0.4.jar",
            combined! {
                Kind::JarClassV1 => "1ebbc2dc9c8c70c0439e5a67a4fc661cfae59e0c2eb882af6458c8a9a422f566",
                Kind::JarRawV1 => "dcb3b652513c2f2ae585e3b8cd3f7d6c21f4f71966a39b943456a50842e35db1",
                Kind::RawSha256 => "b73c80be1099c4756c088cb457a82040509b787519af5c72c9c3d1bff357565e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/tools/plugin-cli/elasticsearch-plugin-cli-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "765ab61850cb53f0fc9972f7e3f86581781ea1d45f8c8aff8725121f0ca7326c",
                Kind::RawSha256 => "732aa986c38a67423270e370077683c523c45fe80fbf354dfad7c1a7cc55ee33",
                Kind::JarRawV1 => "c555330e0416f4d26189fc9b4a01a855b501d6174e67589d1b4f0755ac5b42f8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/tools/security-cli/bcpkix-jdk15on-1.64.jar",
            combined! {
                Kind::RawSha256 => "84669138b1d99143e2c009024f67824ab8d3edb9b05b7591f5ebfb020a4bda71",
                Kind::JarClassV1 => "9dec0393b27cdce1527c823c9360cd93646c5217a6e447a536282f10ea887568",
                Kind::JarRawV1 => "b481944d2254e18a73cb5f98fa2a7ace52d20cb10110f5796dc462ff40e7b028",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/tools/security-cli/bcprov-jdk15on-1.64.jar",
            combined! {
                Kind::RawSha256 => "a4f463ce552b908a722fa198ef4892a226b3225e453f8df10d5c0a5bfe5db6b6",
                Kind::JarRawV1 => "21f2db23f45a78c1aeae1b405cf14fb57cfd0014483046c08c7c0ce843f35d77",
                Kind::JarClassV1 => "e976b92560cd39f483456535bb09bd4aa76fc3d84b3182d47fec40a0e2c7dce3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/lib/tools/security-cli/elasticsearch-security-cli-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "57fa27ff77009b9b650ec95ae94a955f32d5580cee38ec9e69162d351e405981",
                Kind::RawSha256 => "6b395db07831987f18b9556eae60ebace4f7ca5bd179923d64f8ca0960f3f72f",
                Kind::JarClassV1 => "7670dc9e05baba8828e8d005da86fbbad1e1be81eeb53dee8b1e17ffa589ec57",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/aggs-matrix-stats/aggs-matrix-stats-client-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "37633e596eb108f031abf99f743de6035efcde70f8a8973e72ceffc724d94d02",
                Kind::JarClassV1 => "aa2a5bcac58731f1c102036b5030ba2b520728591696dde4dc45c6623127fdea",
                Kind::JarRawV1 => "3cf096f9b1cb2e9b047c0a65688207cd57cce1fbcf60875a73baabac6b173924",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/analysis-common/analysis-common-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "af690d85617a94d8e0566067b21e4f98828d131f1fb9fcaf51add542a40cb41d",
                Kind::RawSha256 => "f3e58511ca0d75f93f62a79edd42f44861e8871f6c9e724b63c883fe749e448c",
                Kind::JarClassV1 => "7e079430c48683162a073207cdbbb2a1c9ab4d1847804ee7d7f09ec0ede6c37b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/constant-keyword/constant-keyword-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "ac8c6274eecb55993c19cd8976490c166df5847c04e4a756b4c137eb2ef5d45f",
                Kind::JarClassV1 => "217598f42ef9a99a0973ee0c2d9d5b42bd620a31619bb3b47d3b9121fb4a2911",
                Kind::RawSha256 => "c91a1f0e92b988c93b9a1ccc09d6c3ba0cf4212ee6b5ff71df4175c64704a2cd",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/frozen-indices/frozen-indices-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "b773bc4d9fb28e0993470d593db345fcd4db1f4dfb17b5c62925eea7acecebc2",
                Kind::JarRawV1 => "59f8936e37d53261f3c437adb87ddf1f65aaa56c50c9a6028c37acd3cfaeb85c",
                Kind::JarClassV1 => "08a4c2ccec8e872873f0e20fc497700d1e7342e628deb04f11d26762b11f6d38",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-common/elasticsearch-dissect-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "95041177b60ad6b80438591597dd00a08188314bfd02b35a7ece3f2ec9b37534",
                Kind::JarRawV1 => "49db02909357ae5cf134bc7390503faa35ad0023a62d2885b35d1b12249e2fc9",
                Kind::RawSha256 => "96342d26d6bfa0e64fcdb0bad54bc6c946c3b8e5b597fd051db3b7f4e6b17402",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-common/elasticsearch-grok-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "3244f3a0b70d8cb7cf4c44e31806c14f038374b85b98dd965a9fee5ca7709d2e",
                Kind::RawSha256 => "2e524c254feecbb5d41b42d4a4d8eb3ed96f3d57b1e7c9fd0ef4839581c49dc1",
                Kind::JarClassV1 => "fee1097201de7908879f65cdaf0fc54bc60a4e023450ca1f9eaf3ffb7b14bc84",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-common/httpclient-4.5.10.jar",
            combined! {
                Kind::RawSha256 => "38b9f16f504928e4db736a433b9cd10968d9ec8d6f5d0e61a64889a689172134",
                Kind::JarClassV1 => "444d77f0054d951ffeec24f16be3aef0ffc76edc546c7970fe4925aa3b71252b",
                Kind::JarRawV1 => "8491995343994e75696217d4f400d2390942dac99c1e3c4a67532f9b9e71c846",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-common/httpcore-4.4.12.jar",
            combined! {
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-common/ingest-common-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "eb3c740d37081b228c6ddfc867fdd54b2a1f9990dfb88e78c39d9712784b1c82",
                Kind::JarClassV1 => "91f73b6eec8d27a640d9c6c2e1b6812640176fbec410cfcf39c7f92621f4e4ff",
                Kind::RawSha256 => "6454b612d8f2c6c073507e50daabf23bfbccc0f6e8f452e9420aa94ed7a5e695",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-common/jcodings-1.0.44.jar",
            combined! {
                Kind::RawSha256 => "49190d6ad09056de57d7ed41ed5b4b105e033557b5dd170702decdcf05ee341a",
                Kind::JarRawV1 => "3ff823499e3d4fd4815efbf4a54311290f5a64592d944e901f8a2940009b35dd",
                Kind::JarClassV1 => "4dd715e0832222e95234af07d22995a0cc40fdc7328ec88b355877d060650532",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-common/joni-2.1.29.jar",
            combined! {
                Kind::JarClassV1 => "e9629b51e39e64cd10c5a7a7e10dbb0c5428ce4006a864da5fba4e1a1ec03ff2",
                Kind::RawSha256 => "aa4b71415682f3d7fa44083cd94a9ec48478ec3b9c30947b4152913d41b1004d",
                Kind::JarRawV1 => "5f5c1894e9780c6e408de3baeee7585886d8fbc3ae73b6af25722912567cdcff",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-geoip/geoip2-2.13.1.jar",
            combined! {
                Kind::RawSha256 => "cd0ef447c0e0466d38bc75c1230b7c79b7aa8d353aa25e14f552ef8f3fd0ff3b",
                Kind::JarRawV1 => "e1db7342c4d478d1d857a8c4a875c2454c6fd9070b1e0a0eea0fe963d294649f",
                Kind::JarClassV1 => "ca8671b9d93804c1c4a51d52661c75b0ced57bb4d0b00568007af6bfb7c2d791",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-geoip/ingest-geoip-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "e1ac2432c4adba4917809c5f5a2b4e46a4b156b3747aa0644e862e27e7b3c973",
                Kind::JarClassV1 => "fffe1d0b7f3c0f97d17c8bfd5ecc0f0f0a46738ce8bfbc847b3af4fd0572eca9",
                Kind::JarRawV1 => "128d53401fc7e76a624b1757d222f703e2770b7e77fd1d9c97fc786d4645f6f3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-geoip/jackson-annotations-2.10.4.jar",
            combined! {
                Kind::JarRawV1 => "c2b037f66af48846c31348472e7028a9ec9c77c0deb1e6040f3b063cb555fd2b",
                Kind::RawSha256 => "9d07ea7ce579a678e7aea61249fa82c46469af9d02c5b5f13e84beb7b0827dbc",
                Kind::JarClassV1 => "a279dee3f1a0902b3fa6105457e11e62024399479dcc6a100657dfeda456f440",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-geoip/jackson-databind-2.10.4.jar",
            combined! {
                Kind::RawSha256 => "55312662a420c71508e6159c86aa41c1694c52e89a1b90dc94bcf4358134005e",
                Kind::JarRawV1 => "5c7a86c671c8ffbddf5c4c9e7d40d38a345f1e8a329eee6d698e29744924a19e",
                Kind::JarClassV1 => "e3152904733e8aed7aff00033d6985c7a40dcc7b4d83b6ad04406431fe82431a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-geoip/maxmind-db-1.3.1.jar",
            combined! {
                Kind::RawSha256 => "cc955e897306baadad7b16d58918bb020c847010d458a603a0e53b5a47bd0961",
                Kind::JarRawV1 => "4edd6528973323c7ca9a679cde8faf0b80b48d04810eefceeb859b69e427e427",
                Kind::JarClassV1 => "02bd49521a6fefcf8ad678b4045934933e891a2904e7c00d435c82ca8270850a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/ingest-user-agent/ingest-user-agent-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "b8acb2156614d4df162cd12d1a3dffe34e302814967439b98993086bd2e35c90",
                Kind::RawSha256 => "a27404207c466e041edbd5d49b8f80b229d01755ca44629a5cefa33a05349339",
                Kind::JarClassV1 => "38447602f227c0912e968e1a43b17c618d2242c477fe2714e4508c70b5feb14e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/commons-codec-1.11.jar",
            combined! {
                Kind::JarRawV1 => "592746b12d517d74550cd9056e12e0d1dd36ede46a87b29a61fc576f7c6abf2d",
                Kind::JarClassV1 => "e7e789b464c215ef83656198d719851f62b4a198f5a3aa03844c4c1f30aa1bce",
                Kind::RawSha256 => "e599d5318e97aa48f42136a2927e6dfa4e8881dff0e6c8e3109ddbbff51d7b7d",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/commons-logging-1.1.3.jar",
            combined! {
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/elasticsearch-rest-client-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "63c408bfa682880f025860b8eeeb3aaf23abd79d50af3d06e10972b9bee3c4c8",
                Kind::RawSha256 => "d9c00a48a62e79354c871ffe3a45c18d6110b99d53424d9a15d8a370c70f091e",
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/elasticsearch-ssl-config-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "a99e47bb8e50a50778c626de194896cbdf814bc12d727db40707952e1ef9f3c4",
                Kind::JarRawV1 => "b588432a36b97a88036e8e4b25950f8345d8ef106b70263c1c6e6e6bdfe593b3",
                Kind::JarClassV1 => "bfa8378ae967a7d0e3dc1ffc52b65da2eb6afe95763c54dbc3e7c97f9169c5f6",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/httpasyncclient-4.1.4.jar",
            combined! {
                Kind::RawSha256 => "50e981a8e567a16ebdad104605b156540a863459fa127b8ba647f310dfc83ef8",
                Kind::JarRawV1 => "45f280f8ed6c9600a41afc2bf6f73dbd96bab37c0cd0bf66e52f916c9c57a9a4",
                Kind::JarClassV1 => "77f6448014b48e685332f32f0aec9c34383bb7a69ea8e2586f2a738418be6669",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/httpclient-4.5.10.jar",
            combined! {
                Kind::JarRawV1 => "8491995343994e75696217d4f400d2390942dac99c1e3c4a67532f9b9e71c846",
                Kind::RawSha256 => "38b9f16f504928e4db736a433b9cd10968d9ec8d6f5d0e61a64889a689172134",
                Kind::JarClassV1 => "444d77f0054d951ffeec24f16be3aef0ffc76edc546c7970fe4925aa3b71252b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/httpcore-4.4.12.jar",
            combined! {
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/httpcore-nio-4.4.12.jar",
            combined! {
                Kind::RawSha256 => "11448f4b5c7f13d9396a67b33aa938d05f660665e0f14fd08e25acfd3c20ae80",
                Kind::JarRawV1 => "24e4f97e5ff3f463dc40dbdd72e84f11c506e479c31e5a966dc92f68367ef631",
                Kind::JarClassV1 => "46e99f783b2e361033e0b5d74193643715b4faa2037d6aaf39aa1bf93fc05adf",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/kibana-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "42a53d242bbbe1adf01250b8c0ae92342dff04b4688471e3d4a4ef404566b618",
                Kind::RawSha256 => "70e2d4bda32a6c8288431e99d5be3172a6ec21e0ab123e44c6bec4ba3edfddb1",
                Kind::JarRawV1 => "ff6bc97a6618a823812da4320d88badef4823079eeaba55c745c88a872c20ba8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/kibana/reindex-client-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "711a6e126ab14e65b90a7bce7adb29004fbb365afe7563aa2160bd47c43cb2ef",
                Kind::JarRawV1 => "26048a4fbe7be04c607d43f08b3faf9e42a81ce9e0ce29480e8505ade2161d9e",
                Kind::RawSha256 => "741e7e3ea769bb7a990ed84dec2d6025bf7c7122c2649e9da083e7f4b5089c26",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-expression/antlr4-runtime-4.5.1-1.jar",
            combined! {
                Kind::JarRawV1 => "74f6c74696df093e7004416f9073ef292b6ecb8f938553f914721b70802b7ec6",
                Kind::JarClassV1 => "7e61d6987054a4d1dbf1616459e780cb57162a62d564f441ca3b5bfd323a1550",
                Kind::RawSha256 => "ffca72bc2a25bb2b0c80a58cee60530a78be17da739bb6c91a8c2e3584ca099e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-expression/asm-5.0.4.jar",
            combined! {
                Kind::JarRawV1 => "6c75e4cc2dd9e51523115291b81a99b91fa95693a4768aaa22c76007fea80b1d",
                Kind::RawSha256 => "896618ed8ae62702521a78bc7be42b7c491a08e6920a15f89a3ecdec31e9a220",
                Kind::JarClassV1 => "26ba53d2e2e49f7f223d46afff57586676b93a934cb1157ddeb244fb1a3b6f96",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-expression/asm-commons-5.0.4.jar",
            combined! {
                Kind::JarClassV1 => "b8d0aabaf89d15aa5a6c32fce16a37f10615d5b753ce7d97449051cd41a9108b",
                Kind::RawSha256 => "532f0ea290b28651b18f03e375f1b5693e87bbf696a25879ea1f1182557486af",
                Kind::JarRawV1 => "74c6818b325f573cad4e4fadf0be091b6a19ddd17cad95f077cd5392a4b23ed8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-expression/asm-tree-5.0.4.jar",
            combined! {
                Kind::RawSha256 => "c3670fa836fac4cde743840f671a7d52a32eb0a844e2824eaedaf3a47f5c8399",
                Kind::JarClassV1 => "70128ccdd0720210ab2ef935cfb59cfaaee509403e3bed572c792a1d615ac06f",
                Kind::JarRawV1 => "d056a33c47afa974bae98a863d8619ec19f84b299c2edb19b725f9224998acb5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-expression/lang-expression-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "006cf5eb0e314732ecb73f902196886d98afa3771e2d591c23fd689af7a00840",
                Kind::RawSha256 => "e22799534712b864f2c06a9e640263760530b91eedeaee63da30ba6795b21d6f",
                Kind::JarClassV1 => "276338e47f30a2276fb108f5a62e4d29ba6aed481c2d4557922e2faf2af340e2",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-expression/lucene-expressions-8.11.1.jar",
            combined! {
                Kind::JarClassV1 => "71173938dd67919a58a272abf6bdd31a0988d3a7e5d40262373b0f116f6123b2",
                Kind::JarRawV1 => "202d97b9c0a70e6c80e31772490c264d33060623d36757d5452b30c7f1bb93d9",
                Kind::RawSha256 => "7b3ac33889a54487794ad7b7afe8c71b77280c37cf148a0cde082ffad2824414",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-mustache/compiler-0.9.6.jar",
            combined! {
                Kind::JarClassV1 => "636d900e125852c79678a168f1b1a285af804a3538bf94907437cd25e5df5e08",
                Kind::RawSha256 => "c4d697fd3619cb616cc5e22e9530c8a4fd4a8e9a76953c0655ee627cb2d22318",
                Kind::JarRawV1 => "07e52d443b6f1819ac0e9ca901d1b6a9232a9220d394d6979542864ad308eee4",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-mustache/lang-mustache-client-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "6d5c77f4dead86af7438a9f1365d93ae4a761ad861bfd8e80d4b132eec58a680",
                Kind::RawSha256 => "c9bec0366b01d21f275bb24314725626282b407f6d0a6af4cba21bdef4b56c12",
                Kind::JarRawV1 => "7ade450c85ec11905e26fcf540094f18b838746f60dc61173d247ef18f27295b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-painless/antlr4-runtime-4.5.3.jar",
            combined! {
                Kind::RawSha256 => "93bca08ec995caeaaf60bdf80035a0be8507fcdabd3c2618fd8c5aab4444a752",
                Kind::JarClassV1 => "8cdfae0e14be3bcd345fbbf3dfe2bee73e5e82ed21c259af47d8cbe13a1dcd29",
                Kind::JarRawV1 => "48d8ab8b98757a6437ada2fa196645abc4be5cb192a3a9f50aa670875c5417a3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-painless/asm-7.2.jar",
            combined! {
                Kind::RawSha256 => "7e6cc9e92eb94d04e39356c6d8144ca058cda961c344a7f62166a405f3206672",
                Kind::JarClassV1 => "f4c640b0e933f55e567e0c6ed31609515a1da985706e87ae597d8914a541c490",
                Kind::JarRawV1 => "2bed47e7fee145b8cf2945dd07fadc2ed02b7e16c6c234c9bf4ee88ea4a6902e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-painless/asm-analysis-7.2.jar",
            combined! {
                Kind::RawSha256 => "be922aae60ff1ff1768e8e6544a38a7f92bd0a6d6b0b9791f94955d1bd453de2",
                Kind::JarRawV1 => "1e219938f4d0f103ea2e9f63dac1dcadb42dd7c16829a9acc6c7e98b1e57aef7",
                Kind::JarClassV1 => "2e62a2125a33e5a8252b34adcd9424f2a1c79ebd97f9731e700bbfc6e6c7af6f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-painless/asm-commons-7.2.jar",
            combined! {
                Kind::RawSha256 => "0e86b8b179c5fb223d1a880a0ff4960b6978223984b94e62e71135f2d8ea3558",
                Kind::JarRawV1 => "55c8209eee3fd2df3c56572fde4c05870c4866c5d7220d7bb2fff5751d57b758",
                Kind::JarClassV1 => "4708669806879cc354718783e58a85ccbb4c28f566e680e1f7b6af65b052089e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-painless/asm-tree-7.2.jar",
            combined! {
                Kind::JarClassV1 => "255dfc62fe4b3073327fb54353d7fee0a1636f3c87907cb43d1500774ab764e3",
                Kind::RawSha256 => "c063f5a67fa03cdc9bd79fd1c2ea6816cc4a19473ecdfbd9e9153b408c6f2656",
                Kind::JarRawV1 => "1ee5d5e55da2307c6bbf3a5d1f84470c460c9b07855e05fd9d656116418e6062",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-painless/asm-util-7.2.jar",
            combined! {
                Kind::JarClassV1 => "4e78cf683c8701f0b16b188e577049a88545d2b8dc38175b59bc583a89f74606",
                Kind::JarRawV1 => "ab429a0e53bb8b36642e3c4739ef4624a31bc852032a64d3045b52c6a1f6e155",
                Kind::RawSha256 => "6e24913b021ffacfe8e7e053d6e0ccc731941148cfa078d4f1ed3d96904530f8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-painless/lang-painless-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "c322a82af6a6a636bdf38ab6f2d8f3cbf0264f2d2ba33062af511c78d2f151ca",
                Kind::JarRawV1 => "d165ea5ae7c1baff387d64c81d9c4df569e4b657b2870327b8091292dfff79ea",
                Kind::JarClassV1 => "5fc5b45187f06124f07dabe9773fd5399fdffca8130083a342eeee127b14a8e0",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/lang-painless/spi/elasticsearch-scripting-painless-spi-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "861d79bad8f4b2ede24bac9c530ee1c3cf5332c6853a910d44ce6aeb682102cd",
                Kind::JarClassV1 => "bd8af6c013f5e874f59969799d3ace10b86fd8536db616fd6d043a6c979329d4",
                Kind::RawSha256 => "b0ced39dedd7d205dad49217b3a6ead7ad6943a23dc42cdf708be1a840bdb31a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/legacy-geo/jts-core-1.15.0.jar",
            combined! {
                Kind::JarClassV1 => "dcfec9f02a26f99933123a3e656e61783e96b304f0c2e22060d98f158832777d",
                Kind::RawSha256 => "00102cde26c457b81fbb0248e4f8845884243caba0dc9b7fb42e0ea877383bc1",
                Kind::JarRawV1 => "4462ea26e83f4af4dcd63f3bf039478a1982bccef562f637865439fb5aa985df",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/legacy-geo/legacy-geo-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "28606d0a1d280dc2c082f2277b799c563dd4c0f7082bfd55cd24bb48825bbad6",
                Kind::RawSha256 => "7c470052b1abdd63a00a9333ee526594f7cdbe30287ff4d6cda7c03d3ba2daa5",
                Kind::JarClassV1 => "3c8db452302434296a9f650a994e96efd392a19088a0fb24bfa9cf02a7fedbe6",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/legacy-geo/lucene-spatial-extras-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "68c7e499c30111de061cbdc87998c48f92d1a8377774723b30ae069326b27a67",
                Kind::RawSha256 => "ae82f78c01fe969930ecea104a9dc78bc210fc45b55c3d695b66b80468cb69e3",
                Kind::JarClassV1 => "7d0176090e6d72b340cebbfd307bcee0fb9b6c6cbdbe8e45276fcb76b0f67a4c",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/legacy-geo/spatial4j-0.7.jar",
            combined! {
                Kind::RawSha256 => "9adccb1d87f7e0be70567b952c65552607e2dcbde32a1579a8a639bdcfa1a3c8",
                Kind::JarClassV1 => "d46c77130b99e0fe578ac7ae87e44bff6e3d42e16193bb02177064c4c1a92ef9",
                Kind::JarRawV1 => "998f9109b0c4d61ef344693eedbffd672cf0a0602e271e18ed9a8101022fd1a3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/mapper-extras/mapper-extras-client-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "e3242ac1681b2d2047ebba03bf82b7b973c12f29d694b43bc3020d7f4630c3a3",
                Kind::JarClassV1 => "f62a000140ab8c7c0c9292c74c52dd7e39f3b22a2e2e7a616482c44a4122003a",
                Kind::JarRawV1 => "74b9a468e839c0718904e4efd985ac81375984057a9c9314c33cfe12d9d1cebf",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/mapper-version/mapper-version-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "584980ce301e3c9d4c68f1e122b1a7f9b1ccbb7c3e8e7e188b8a60aaded6e8ce",
                Kind::JarClassV1 => "86555ae5b34c6c1553e9bf2eaf06657f26d58ba64dd24b11ef1eb3d5cad6dc27",
                Kind::RawSha256 => "f7db64050bd0b7650699b8d163817f53783cd1f859b4aa2c2e85ca5bf7665ee2",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/parent-join/parent-join-client-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "cfb4c516599cd28807b7e0d63c3391a4d95ea44782d587d6fee2d83978b41804",
                Kind::JarRawV1 => "8eec2515f7c5b352ccdfa909711158c9d7ba0526a61b41244a3ec03c7d2af286",
                Kind::JarClassV1 => "24f28b07f5aec5a739d14b2c3a825e92685e75a301040a96bdcbd9fb8185f4d4",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/percolator/percolator-client-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "0220bf02f5c0594403a96dc9f6147a9ade5971874ce375d9e523501de3e768ef",
                Kind::JarRawV1 => "d7c346e4c578e79b0b8802888e872ef4d7db80bd65b0dac53d6fdd522fa1fa76",
                Kind::JarClassV1 => "4dd6d980a2da4dd00e1e6b3b127e862656fc3bedd4fe8cd0c5dbe080c12890ce",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/rank-eval/rank-eval-client-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "9eb2df8ac89660801a866582cd313b96cf4afa8d0649bdc07f459282dcd9f58f",
                Kind::JarClassV1 => "564235e29238445483fc09203c00c8065ab7c2a65572b38283b20832ae338973",
                Kind::RawSha256 => "4e0dcec867815ec99ebddff475aa608e68c0a8f5d8bcf624118fefebc2ff6527",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/commons-codec-1.11.jar",
            combined! {
                Kind::RawSha256 => "e599d5318e97aa48f42136a2927e6dfa4e8881dff0e6c8e3109ddbbff51d7b7d",
                Kind::JarClassV1 => "e7e789b464c215ef83656198d719851f62b4a198f5a3aa03844c4c1f30aa1bce",
                Kind::JarRawV1 => "592746b12d517d74550cd9056e12e0d1dd36ede46a87b29a61fc576f7c6abf2d",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/commons-logging-1.1.3.jar",
            combined! {
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/elasticsearch-rest-client-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "63c408bfa682880f025860b8eeeb3aaf23abd79d50af3d06e10972b9bee3c4c8",
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
                Kind::RawSha256 => "d9c00a48a62e79354c871ffe3a45c18d6110b99d53424d9a15d8a370c70f091e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/elasticsearch-ssl-config-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "bfa8378ae967a7d0e3dc1ffc52b65da2eb6afe95763c54dbc3e7c97f9169c5f6",
                Kind::RawSha256 => "a99e47bb8e50a50778c626de194896cbdf814bc12d727db40707952e1ef9f3c4",
                Kind::JarRawV1 => "b588432a36b97a88036e8e4b25950f8345d8ef106b70263c1c6e6e6bdfe593b3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/httpasyncclient-4.1.4.jar",
            combined! {
                Kind::JarRawV1 => "45f280f8ed6c9600a41afc2bf6f73dbd96bab37c0cd0bf66e52f916c9c57a9a4",
                Kind::JarClassV1 => "77f6448014b48e685332f32f0aec9c34383bb7a69ea8e2586f2a738418be6669",
                Kind::RawSha256 => "50e981a8e567a16ebdad104605b156540a863459fa127b8ba647f310dfc83ef8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/httpclient-4.5.10.jar",
            combined! {
                Kind::JarClassV1 => "444d77f0054d951ffeec24f16be3aef0ffc76edc546c7970fe4925aa3b71252b",
                Kind::RawSha256 => "38b9f16f504928e4db736a433b9cd10968d9ec8d6f5d0e61a64889a689172134",
                Kind::JarRawV1 => "8491995343994e75696217d4f400d2390942dac99c1e3c4a67532f9b9e71c846",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/httpcore-4.4.12.jar",
            combined! {
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/httpcore-nio-4.4.12.jar",
            combined! {
                Kind::RawSha256 => "11448f4b5c7f13d9396a67b33aa938d05f660665e0f14fd08e25acfd3c20ae80",
                Kind::JarClassV1 => "46e99f783b2e361033e0b5d74193643715b4faa2037d6aaf39aa1bf93fc05adf",
                Kind::JarRawV1 => "24e4f97e5ff3f463dc40dbdd72e84f11c506e479c31e5a966dc92f68367ef631",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/reindex/reindex-client-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "26048a4fbe7be04c607d43f08b3faf9e42a81ce9e0ce29480e8505ade2161d9e",
                Kind::JarClassV1 => "711a6e126ab14e65b90a7bce7adb29004fbb365afe7563aa2160bd47c43cb2ef",
                Kind::RawSha256 => "741e7e3ea769bb7a990ed84dec2d6025bf7c7122c2649e9da083e7f4b5089c26",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/repositories-metering-api/repositories-metering-api-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "bb8cfdb5dfcc1e277cf052d4ccb20a841da06c048f8dcbb2c1e1610478453de0",
                Kind::RawSha256 => "b2e9914caf658a81fa9d72df639d834659d6f56a168437feab90f72e9f2e6e56",
                Kind::JarClassV1 => "d34b3d50656d42f4885c86d33b78b46d07f069207233a782e8989e7d5312c533",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/repository-encrypted/repository-encrypted-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "7d01935ac5d0008196fb6a7818d5c391dcdd8a4f89bd33878450fb32252d4243",
                Kind::JarClassV1 => "cef5f67b32aaa97c9f9c3a91cf1fdbfcc2d3da385d8761857498a5695fc19e58",
                Kind::JarRawV1 => "8dd9265ea48127485e4b9ed48c0738999525a10daab3c2e807cc3c67b9f00af6",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/repository-url/commons-codec-1.11.jar",
            combined! {
                Kind::JarClassV1 => "e7e789b464c215ef83656198d719851f62b4a198f5a3aa03844c4c1f30aa1bce",
                Kind::JarRawV1 => "592746b12d517d74550cd9056e12e0d1dd36ede46a87b29a61fc576f7c6abf2d",
                Kind::RawSha256 => "e599d5318e97aa48f42136a2927e6dfa4e8881dff0e6c8e3109ddbbff51d7b7d",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/repository-url/commons-logging-1.1.3.jar",
            combined! {
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/repository-url/httpclient-4.5.10.jar",
            combined! {
                Kind::RawSha256 => "38b9f16f504928e4db736a433b9cd10968d9ec8d6f5d0e61a64889a689172134",
                Kind::JarRawV1 => "8491995343994e75696217d4f400d2390942dac99c1e3c4a67532f9b9e71c846",
                Kind::JarClassV1 => "444d77f0054d951ffeec24f16be3aef0ffc76edc546c7970fe4925aa3b71252b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/repository-url/httpcore-4.4.12.jar",
            combined! {
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/repository-url/log4j-1.2-api-2.17.1.jar",
            combined! {
                Kind::JarClassV1 => "6e705c0891b1027df1444ecf13cce77d48cfa4b039bf2fbb596937a2f41bdfc1",
                Kind::RawSha256 => "ca3e9150f95c31d15b9680a609b8817f8549bd395591c5ca55957d1ef0f464d6",
                Kind::JarRawV1 => "b6fa23d8eb7cad6b08c18f318a869f85778b0cdc9e9eac56a9fde7db263fdd72",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/repository-url/repository-url-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "1063a2a9244edc3121f505836286da81d0a6f5d40a93dce02cd27e6461a63015",
                Kind::JarClassV1 => "cea3ef483be29377186d52fdb1a2300a53cb2a59fe6f046a5744b9e6a304ee18",
                Kind::RawSha256 => "713a99c6cf99dcf95abdd048a9dc93e9c1af526b27a30a8ff21fae65d1dc60d5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/runtime-fields-common/elasticsearch-dissect-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "96342d26d6bfa0e64fcdb0bad54bc6c946c3b8e5b597fd051db3b7f4e6b17402",
                Kind::JarClassV1 => "95041177b60ad6b80438591597dd00a08188314bfd02b35a7ece3f2ec9b37534",
                Kind::JarRawV1 => "49db02909357ae5cf134bc7390503faa35ad0023a62d2885b35d1b12249e2fc9",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/runtime-fields-common/elasticsearch-grok-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "fee1097201de7908879f65cdaf0fc54bc60a4e023450ca1f9eaf3ffb7b14bc84",
                Kind::JarRawV1 => "3244f3a0b70d8cb7cf4c44e31806c14f038374b85b98dd965a9fee5ca7709d2e",
                Kind::RawSha256 => "2e524c254feecbb5d41b42d4a4d8eb3ed96f3d57b1e7c9fd0ef4839581c49dc1",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/runtime-fields-common/jcodings-1.0.44.jar",
            combined! {
                Kind::JarClassV1 => "4dd715e0832222e95234af07d22995a0cc40fdc7328ec88b355877d060650532",
                Kind::JarRawV1 => "3ff823499e3d4fd4815efbf4a54311290f5a64592d944e901f8a2940009b35dd",
                Kind::RawSha256 => "49190d6ad09056de57d7ed41ed5b4b105e033557b5dd170702decdcf05ee341a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/runtime-fields-common/joni-2.1.29.jar",
            combined! {
                Kind::JarRawV1 => "5f5c1894e9780c6e408de3baeee7585886d8fbc3ae73b6af25722912567cdcff",
                Kind::JarClassV1 => "e9629b51e39e64cd10c5a7a7e10dbb0c5428ce4006a864da5fba4e1a1ec03ff2",
                Kind::RawSha256 => "aa4b71415682f3d7fa44083cd94a9ec48478ec3b9c30947b4152913d41b1004d",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/runtime-fields-common/runtime-fields-common-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "9d323f4300e50014348a54c6ff6c8177ac919c10cbc1267edc7c624b83ef937a",
                Kind::RawSha256 => "d6cfc85fdcb94a4b32a779c870ea9eec4790cd307b460669d3c95fca51edeee7",
                Kind::JarRawV1 => "efa1da8e2684300538b2d9ae0365c4c32e64ee170e4dffd7cd82ae3c0e376b58",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/search-business-rules/search-business-rules-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "ddacc7d3ce4db48d0369dc53208063a83ee32e67b9482bb1635f81a81a26f304",
                Kind::JarClassV1 => "7aa97a9c72b5be6a324790f600c9ea8dba51f900d767431e2ffdc7915ff67118",
                Kind::RawSha256 => "9c68e2dfa4b7f8f1dab8474fc4c8311c6aa3f55b89bd45a936bcebe44b94de65",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/searchable-snapshots/preallocate-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "2290faa2039dcfdbc3d764aec7be116c884aeafefa5f054358ef480bbe6905d7",
                Kind::JarClassV1 => "195c7a82e8677613fa70a43488c101b19df6cf185009b4ea3733e653d41e4c18",
                Kind::RawSha256 => "d0e7855afd83bc2735dadce43cb122caf93c54af6e85b62080d37e4da96f4941",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/searchable-snapshots/searchable-snapshots-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "fe6e646a86f5736abeb1d19695ada5361b697a852c92fb49a0a4dc36a9ddc564",
                Kind::JarClassV1 => "8c0607e5669aa5a138bbc0fe86f36b2e5ad6b3fbdeffe85c1dfb2202cd8d5cf0",
                Kind::RawSha256 => "3e71a33307904b8b53cf6235e2407df14eb267826b1180c14546f7d1eb201227",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/snapshot-repo-test-kit/snapshot-repo-test-kit-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "945a94d1d4802f58a962dcaf412ad291e34f55ed4d3e3e67847e6226c7e32d05",
                Kind::RawSha256 => "d4e2848e4daf82a9e9bf9bc62de09035fbbed93846832f78ed35de241f5cb2e1",
                Kind::JarRawV1 => "009449622dd37c058ab1649cf876a5f3cc19775b1d503e8e322098fae9912ef1",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/spatial/spatial-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "c05bdf362fa3b89754ea149f1a34ba8760b510e9eb1ebc2723ef375f22588584",
                Kind::JarClassV1 => "474a3b7995bbefb730cfa28ed3dc980659ed475c750a1f733438507785be46b0",
                Kind::RawSha256 => "47e5074bb981246e0146bd9444de3aeca0028fe84005d86b829ccfc4f2529c99",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transform/transform-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "baf55703e9a8aad54db778f4429484a5bc5aaf2971ff6a1e323e2f8237478a6a",
                Kind::RawSha256 => "45e4ca07f745bc9f246c8b477ce7a1317868fb8ee9e34e737fa21c1187c6cce7",
                Kind::JarRawV1 => "612515a86467ea86afe48ef6d50c47e00e0add887b2b741a17929be2a8a010f1",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transport-netty4/netty-buffer-4.1.66.Final.jar",
            combined! {
                Kind::JarClassV1 => "095bdd99f38fd8684d9905ec787e2fecd63f30b39d1c2ea82599dccda1fc6350",
                Kind::JarRawV1 => "7a45231927f10d9e945ce1983a50abc2e6576d02ca3b71f424c2b7da002897c3",
                Kind::RawSha256 => "99af46a08546da9e03cb5cd6e3daac624771bb08663f304d60a988e27da59cef",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transport-netty4/netty-codec-4.1.66.Final.jar",
            combined! {
                Kind::JarRawV1 => "32aaa4a067fbada3917725846b79a628789492134151fc6e4e02a832ccb4edca",
                Kind::JarClassV1 => "5d13843d3cc59ac63816faa792968531c346e65e43bf02fb84852a843476fcbe",
                Kind::RawSha256 => "d852eab012b0f06c94a625f96404e32e8fe829708a84e10ae71b2d8dfab47f9e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transport-netty4/netty-codec-http-4.1.66.Final.jar",
            combined! {
                Kind::RawSha256 => "b30b35c69bfe39497aeaf2c7a99bcaea68a9d3eaeee403ecb597cd14f5df4205",
                Kind::JarRawV1 => "53649e231f6c8efcf969a56a8740b8a9691c798e2f702c36c43248380ece9c82",
                Kind::JarClassV1 => "efa8ab0228ff376cd4aa65b1bd01dcc3e09b62edef644f82285eee62f25ca884",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transport-netty4/netty-common-4.1.66.Final.jar",
            combined! {
                Kind::JarRawV1 => "fe3cc3290e4c78f586f75bb0005537fef0d0d3aec7ed392f5c9af70e205c0f30",
                Kind::JarClassV1 => "82a93c08041b3e7aae6c416add0b932473e9baf7bc90f6372497c1cb331f9296",
                Kind::RawSha256 => "bf7e66d832e62dd2cbe7802a3d45ece7f8c6de2958e69d85fae0149cb0820459",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transport-netty4/netty-handler-4.1.66.Final.jar",
            combined! {
                Kind::RawSha256 => "b67da9271458afb434478d86be9f2736a5be2981e49eda2429a9962b10684f7f",
                Kind::JarRawV1 => "dbb746f639fc87411bed4b6c2704c6adc48ca2c42b55bf599bfc9f04767e5c7d",
                Kind::JarClassV1 => "3048985b1473676ac51863c87eb6aa0423e514e61a5d4970691e38446b6c7627",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transport-netty4/netty-resolver-4.1.66.Final.jar",
            combined! {
                Kind::RawSha256 => "e89040e9a760f13ec6c05175d633254dd4ea603b204f7a442744485b3c46d1eb",
                Kind::JarRawV1 => "b7a02681d5dd3fb6bde41211c65190f972403d4a4aeb6671e7c10df2863e2484",
                Kind::JarClassV1 => "a16385ad1c71ab7559e370f72c9a4c54d0621a183a11ae859daeddf82214faa9",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transport-netty4/netty-transport-4.1.66.Final.jar",
            combined! {
                Kind::JarRawV1 => "2b8550b522a62218448adf9f83f20f8b3e165c54db9a7ecfc2ec31d4b3256474",
                Kind::JarClassV1 => "1ab9065be9eae31a9fd7614636335f53a006af27570713344fdaf046ca3e386f",
                Kind::RawSha256 => "59e0c2f8e55e0c1d5df254226395a865af87a9e11f8b96d2dff92660b1dbcfc1",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/transport-netty4/transport-netty4-client-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "a5774f098f89b94ee52d7e88f724d9cb3ec3c0febd1a3b3e783fe5fe99e6286f",
                Kind::JarRawV1 => "f7289c1f18aee88c9d5bae40e417a62d9a6af9e86146cb74d29a65ba5c1c2c94",
                Kind::JarClassV1 => "c3db4ab15e88f3c984aba38a86ab6cf5466649d998e3a4c5d60ef6ca344edf24",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/unsigned-long/unsigned-long-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "883cbd7ea3f95a6c1d2031d36d39a5b7913685573d66e8d212b36a505636f525",
                Kind::RawSha256 => "d28542e51e290de6a6d945e254ecb3b5e68db901eef04bd564f3f2caab1f95a3",
                Kind::JarRawV1 => "fba6ec1993556a5e5be3f9ec479ceba66b41c12a5c80f19cfc92584a28e49ef9",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/vector-tile/log4j-slf4j-impl-2.17.1.jar",
            combined! {
                Kind::JarRawV1 => "a78204ec2fbf25286ad7f7c7d497843d9758c49b55895c8a413998b74a10428e",
                Kind::RawSha256 => "e9a03720e5d5076009c2530635da9d08485e28a0b0ec20708dadc51afb78e41e",
                Kind::JarClassV1 => "9ad437ba29a9f5f245d4ccd6fc067aaa9a4e02dc7c677d61ffbc395738971862",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/vector-tile/mapbox-vector-tile-3.1.0.jar",
            combined! {
                Kind::JarRawV1 => "18ee573e68b4620946d02a130f31718abaabf7d9e311420970fc00690aae1472",
                Kind::JarClassV1 => "77f4b7af48f79b5f302a9b678a1b7de61910d51184c4987c811e31d1ffdbbc02",
                Kind::RawSha256 => "e2bb4c89fae43c6057a88f5d3b11256e78a255467e31654672d72c7b228de435",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/vector-tile/protobuf-java-3.16.1.jar",
            combined! {
                Kind::JarRawV1 => "a974aec8e7ebd6b9f7e25fcf16ebcd4eb9bfbf1ae86c5fcc4c0c8c6cfe96fd4b",
                Kind::RawSha256 => "7b845a34210acde78b7f77977b3724988b9c60b2dce7a93a9afbbb1fee7978c4",
                Kind::JarClassV1 => "90deaba4367c68928ccdb81bb72f3567e6923ffa0b67349e7a55422c63e54cab",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/vector-tile/slf4j-api-1.6.2.jar",
            combined! {
                Kind::JarRawV1 => "51bf440ef88e6f86d2bb6097c537b5afaf8ca93a7fe4a3ee1a203113a6f279d3",
                Kind::JarClassV1 => "af24a6f8c98dd2a81bfbfa9503452c31b64900b76e33a258e102086f661c1133",
                Kind::RawSha256 => "c26413df1741466d8bec549a4b7f6480acb2ac928b156be8c5e3feaf9cba1b59",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/vector-tile/vector-tile-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "8b087ed02c6c4bd7789b9f017f48570bee92eb91611cfde173d9fb0f470f2da8",
                Kind::JarRawV1 => "f5b95db504f468ea3e86f6e4ade6e598cb7360045cdebd89f82809ce20a39b75",
                Kind::JarClassV1 => "2d4b37384b5b7156d81f10c0ebbb0807d852f8b9c8a0da57a65e327436f2f9bc",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/vectors/vectors-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "164916639ce59646fb0cbefd02b2967ee9531f4d29122b6b2eaf7d0194001cd9",
                Kind::JarClassV1 => "bbb183f5d386de9cb701906839429fe12fa110ab45c1a3e08a4b94be74d1c3b6",
                Kind::RawSha256 => "5aff8933cb6eb9985914258cb89a4722b158216bd57d923761ccd89cefc84f10",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/wildcard/wildcard-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "fbd8ec880619f04aa35da99cd53f779f2ea1fb01fc27cbaf743a4f12dac7e680",
                Kind::JarClassV1 => "4dd4aedc4cb4264c4c904696dca9786a4902c0c988579977b14d308e40d348e2",
                Kind::JarRawV1 => "95a545a61985d0df1cff3211a55c54297565c190bd67be224ba8d47730ca2b44",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-aggregate-metric/x-pack-aggregate-metric-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "a8763e21383aa5c7bffc6ffea3201664d59b7aaad4ddaa8119d50d8fbad69752",
                Kind::JarRawV1 => "f0df888e486255f687128ec726a27c70e222255d3b2b9b49da008a0e7eeb57a8",
                Kind::JarClassV1 => "13cc85a8acc799fe2c6333cc974ef2cd1e011ca6dd89f4c59297e326543c9e8b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-analytics/commons-math3-3.6.1.jar",
            combined! {
                Kind::JarRawV1 => "de8a86e786ccdfae373551eee29b07428acdc7ac9eeb495f5b23fed90f36ff35",
                Kind::RawSha256 => "1e56d7b058d28b65abd256b8458e3885b674c1d588fa43cd7d1cbb9c7ef2b308",
                Kind::JarClassV1 => "6bf92ce1f74505c615e5224765586998b0f9aecfa2d790479cd668f28d7101dd",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-analytics/x-pack-analytics-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "a6bb5d9e9401e4bfc3d5eab10c860c14fc60d0f9ef5c3894e0ca38a825690c26",
                Kind::RawSha256 => "a52fe240258c4a2d5c14b959f42a0d560bb30aa10c4d590aa0c7c1f957248906",
                Kind::JarClassV1 => "8db21035ab59196dedcfbf9a6096728086200cc0c755f4fa90d8124df2858a01",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-async-search/x-pack-async-search-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "e079e35651e243360c05f52194dbbef2493b3d68a9f0fa74fbf20b490e4518ac",
                Kind::JarRawV1 => "8f9f72027835a84c83cde740a67d1b8c37638d10056d460adae20504126653e6",
                Kind::RawSha256 => "9aa95e2c8ad208c533424446cd710bf0e38c3dcd11579a0e6190d5be49935038",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-async/x-pack-async-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "e6ca4fd1c07943e40269fd61cc4e383ccaf00993abb4e720ea340ef4115a37c2",
                Kind::JarRawV1 => "c4d7f0c9958d238bc3c33119734feb25169eeb069d112748eb05f562fd6daba1",
                Kind::JarClassV1 => "6e0eec7f2d2d6b49f57719bdae004760dae874521624b0e5351bcfedb41d4766",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-autoscaling/x-pack-autoscaling-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "659f90a22011c15ebd4de73e840906c4f8a5c3e7f271c0826850a29209d812b6",
                Kind::JarRawV1 => "b45294ab1a30a32f004d51d4af3dd7eaf423b99f8e3fada77e4b6bfead501a19",
                Kind::JarClassV1 => "731ed7d14dfd6cd3fade80480d7659e5e2dfa97617b1bb6112fc66db390c9b55",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ccr/x-pack-ccr-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "68943c849ffb045e0cbfe4ab491ccd715d720720004650ba9f868e0d0163ad7a",
                Kind::RawSha256 => "60bab88d1ab2e3dd2effa36c2987a23c10c9f995da42407ce703dc48def384e5",
                Kind::JarRawV1 => "d16d97532c3e16845866b52a16b86aff42cb5335da5a34779a836e75d75fe342",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/commons-codec-1.11.jar",
            combined! {
                Kind::JarClassV1 => "e7e789b464c215ef83656198d719851f62b4a198f5a3aa03844c4c1f30aa1bce",
                Kind::RawSha256 => "e599d5318e97aa48f42136a2927e6dfa4e8881dff0e6c8e3109ddbbff51d7b7d",
                Kind::JarRawV1 => "592746b12d517d74550cd9056e12e0d1dd36ede46a87b29a61fc576f7c6abf2d",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/commons-logging-1.1.3.jar",
            combined! {
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/elasticsearch-nio-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "b13ebccdef2c07f869c081d0bf02ae5b3dc5dbaf7f9d1c512339a853147b7d94",
                Kind::RawSha256 => "a020990fce016cc6163b4c7a902b70e338f3fff1af30ead4aef28a761ad92139",
                Kind::JarClassV1 => "59e545296245fa7bd02c59addae9d6838dbf9d617a66e4b6e008709284394633",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/elasticsearch-ssl-config-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "b588432a36b97a88036e8e4b25950f8345d8ef106b70263c1c6e6e6bdfe593b3",
                Kind::JarClassV1 => "bfa8378ae967a7d0e3dc1ffc52b65da2eb6afe95763c54dbc3e7c97f9169c5f6",
                Kind::RawSha256 => "a99e47bb8e50a50778c626de194896cbdf814bc12d727db40707952e1ef9f3c4",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/httpasyncclient-4.1.4.jar",
            combined! {
                Kind::JarClassV1 => "77f6448014b48e685332f32f0aec9c34383bb7a69ea8e2586f2a738418be6669",
                Kind::RawSha256 => "50e981a8e567a16ebdad104605b156540a863459fa127b8ba647f310dfc83ef8",
                Kind::JarRawV1 => "45f280f8ed6c9600a41afc2bf6f73dbd96bab37c0cd0bf66e52f916c9c57a9a4",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/httpclient-4.5.10.jar",
            combined! {
                Kind::RawSha256 => "38b9f16f504928e4db736a433b9cd10968d9ec8d6f5d0e61a64889a689172134",
                Kind::JarClassV1 => "444d77f0054d951ffeec24f16be3aef0ffc76edc546c7970fe4925aa3b71252b",
                Kind::JarRawV1 => "8491995343994e75696217d4f400d2390942dac99c1e3c4a67532f9b9e71c846",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/httpcore-4.4.12.jar",
            combined! {
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/httpcore-nio-4.4.12.jar",
            combined! {
                Kind::JarRawV1 => "24e4f97e5ff3f463dc40dbdd72e84f11c506e479c31e5a966dc92f68367ef631",
                Kind::RawSha256 => "11448f4b5c7f13d9396a67b33aa938d05f660665e0f14fd08e25acfd3c20ae80",
                Kind::JarClassV1 => "46e99f783b2e361033e0b5d74193643715b4faa2037d6aaf39aa1bf93fc05adf",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/log4j-1.2-api-2.17.1.jar",
            combined! {
                Kind::JarClassV1 => "6e705c0891b1027df1444ecf13cce77d48cfa4b039bf2fbb596937a2f41bdfc1",
                Kind::RawSha256 => "ca3e9150f95c31d15b9680a609b8817f8549bd395591c5ca55957d1ef0f464d6",
                Kind::JarRawV1 => "b6fa23d8eb7cad6b08c18f318a869f85778b0cdc9e9eac56a9fde7db263fdd72",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/netty-buffer-4.1.66.Final.jar",
            combined! {
                Kind::JarRawV1 => "7a45231927f10d9e945ce1983a50abc2e6576d02ca3b71f424c2b7da002897c3",
                Kind::JarClassV1 => "095bdd99f38fd8684d9905ec787e2fecd63f30b39d1c2ea82599dccda1fc6350",
                Kind::RawSha256 => "99af46a08546da9e03cb5cd6e3daac624771bb08663f304d60a988e27da59cef",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/netty-codec-4.1.66.Final.jar",
            combined! {
                Kind::JarRawV1 => "32aaa4a067fbada3917725846b79a628789492134151fc6e4e02a832ccb4edca",
                Kind::JarClassV1 => "5d13843d3cc59ac63816faa792968531c346e65e43bf02fb84852a843476fcbe",
                Kind::RawSha256 => "d852eab012b0f06c94a625f96404e32e8fe829708a84e10ae71b2d8dfab47f9e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/netty-codec-http-4.1.66.Final.jar",
            combined! {
                Kind::RawSha256 => "b30b35c69bfe39497aeaf2c7a99bcaea68a9d3eaeee403ecb597cd14f5df4205",
                Kind::JarRawV1 => "53649e231f6c8efcf969a56a8740b8a9691c798e2f702c36c43248380ece9c82",
                Kind::JarClassV1 => "efa8ab0228ff376cd4aa65b1bd01dcc3e09b62edef644f82285eee62f25ca884",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/netty-common-4.1.66.Final.jar",
            combined! {
                Kind::JarClassV1 => "82a93c08041b3e7aae6c416add0b932473e9baf7bc90f6372497c1cb331f9296",
                Kind::JarRawV1 => "fe3cc3290e4c78f586f75bb0005537fef0d0d3aec7ed392f5c9af70e205c0f30",
                Kind::RawSha256 => "bf7e66d832e62dd2cbe7802a3d45ece7f8c6de2958e69d85fae0149cb0820459",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/netty-handler-4.1.66.Final.jar",
            combined! {
                Kind::RawSha256 => "b67da9271458afb434478d86be9f2736a5be2981e49eda2429a9962b10684f7f",
                Kind::JarRawV1 => "dbb746f639fc87411bed4b6c2704c6adc48ca2c42b55bf599bfc9f04767e5c7d",
                Kind::JarClassV1 => "3048985b1473676ac51863c87eb6aa0423e514e61a5d4970691e38446b6c7627",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/netty-resolver-4.1.66.Final.jar",
            combined! {
                Kind::JarRawV1 => "b7a02681d5dd3fb6bde41211c65190f972403d4a4aeb6671e7c10df2863e2484",
                Kind::JarClassV1 => "a16385ad1c71ab7559e370f72c9a4c54d0621a183a11ae859daeddf82214faa9",
                Kind::RawSha256 => "e89040e9a760f13ec6c05175d633254dd4ea603b204f7a442744485b3c46d1eb",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/netty-transport-4.1.66.Final.jar",
            combined! {
                Kind::RawSha256 => "59e0c2f8e55e0c1d5df254226395a865af87a9e11f8b96d2dff92660b1dbcfc1",
                Kind::JarClassV1 => "1ab9065be9eae31a9fd7614636335f53a006af27570713344fdaf046ca3e386f",
                Kind::JarRawV1 => "2b8550b522a62218448adf9f83f20f8b3e165c54db9a7ecfc2ec31d4b3256474",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/transport-netty4-client-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "a5774f098f89b94ee52d7e88f724d9cb3ec3c0febd1a3b3e783fe5fe99e6286f",
                Kind::JarRawV1 => "f7289c1f18aee88c9d5bae40e417a62d9a6af9e86146cb74d29a65ba5c1c2c94",
                Kind::JarClassV1 => "c3db4ab15e88f3c984aba38a86ab6cf5466649d998e3a4c5d60ef6ca344edf24",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/transport-nio-client-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "7fcc81cdd9bf246096696f2bec22a7e8e06811d2976e37065c46bc913d6edb82",
                Kind::JarClassV1 => "4ba6b44c5a67263597e9219886e58ad1946408411f1687831967b22ce967cdf8",
                Kind::RawSha256 => "3b14530280c0dbc41424a66da3082224a1de7291273181f9fbac9cfb2ded051d",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/unboundid-ldapsdk-4.0.8.jar",
            combined! {
                Kind::JarRawV1 => "c8aad0122c5a74a6831693beb2565ab24feea8ceb8b31b2e19d578a3f53526ec",
                Kind::RawSha256 => "de2e896ab2989dee25f67fe2a3aaf789da0ae29e33aa86c9a359acb5bf9ca1fd",
                Kind::JarClassV1 => "48e4718c28e7303c0922cc86b6508d69ed84e225dd6b12ec2fb76ee5cd390172",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-core/x-pack-core-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "5bfc45f9b5df6a2a5238d5d189327c9efc54b71dc82d28b0e807f6349637b337",
                Kind::RawSha256 => "dc0cbf17acbfd1095b6e3660878e3b88a571a273e374aa4cd32260c311e7b432",
                Kind::JarClassV1 => "6476b515ad3684089660ce3b4057120e9e89528c38b1ceeb61a88313a00ffad4",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-data-streams/x-pack-data-streams-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "5edc3dc064069cd5f5d9b2c57bb5665ad9e6c47a4f64f755a705f79d758e42cc",
                Kind::RawSha256 => "427171d0da13de5f8b6f701b8f017d7ce0c479bc856d0eb68d717d09fd376577",
                Kind::JarRawV1 => "d7f5b4572055abf2cd11f87eaaababe01b1702bfeea4d8245cafb72fdd4fa4f0",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-deprecation/elasticsearch-rest-client-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
                Kind::RawSha256 => "d9c00a48a62e79354c871ffe3a45c18d6110b99d53424d9a15d8a370c70f091e",
                Kind::JarRawV1 => "63c408bfa682880f025860b8eeeb3aaf23abd79d50af3d06e10972b9bee3c4c8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-deprecation/elasticsearch-rest-client-sniffer-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "02aef50dd3f51a2d2cb8ad8bd087f68fdbb2c9b0e5bb398a3cc154ec16c25043",
                Kind::RawSha256 => "572370053f371ce51a5ccc98e7960b98313a52b2b03d461830e3dcc289a95428",
                Kind::JarClassV1 => "a1dd5002b52f0cc9b2f37176ce492036514df0e26fc693b0572a549d53969875",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-deprecation/x-pack-deprecation-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "305185099b4913143b351adeed5f601bea3e452177ee1262b8a5226ee4cb10e9",
                Kind::RawSha256 => "de025251fb5479c4bfa95bb485c8feb6ae9f8c9a289d089f7294a8d10c1b8eee",
                Kind::JarClassV1 => "4578d8b8520ca7609749fa1426eb43b9dccc4aa6c28a0ba27021e9b3a4144aa8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-deprecation/x-pack-monitoring-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "a49f26ce4dc6c52556d7705e9305e9826593a9fd34bcecf38ea0e65daef63452",
                Kind::JarClassV1 => "b141c17e941c95d6820829e9854ae07b093907862e425980bdefae57894fda47",
                Kind::JarRawV1 => "505c91488b9c7f0f05a6568120f4c107d54d3deac1a9a782d21a2c2e337f29b3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-enrich/elasticsearch-rest-client-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "d9c00a48a62e79354c871ffe3a45c18d6110b99d53424d9a15d8a370c70f091e",
                Kind::JarRawV1 => "63c408bfa682880f025860b8eeeb3aaf23abd79d50af3d06e10972b9bee3c4c8",
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-enrich/reindex-client-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "741e7e3ea769bb7a990ed84dec2d6025bf7c7122c2649e9da083e7f4b5089c26",
                Kind::JarClassV1 => "711a6e126ab14e65b90a7bce7adb29004fbb365afe7563aa2160bd47c43cb2ef",
                Kind::JarRawV1 => "26048a4fbe7be04c607d43f08b3faf9e42a81ce9e0ce29480e8505ade2161d9e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-enrich/x-pack-enrich-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "05eac1100bd00d5b13cd75cf7fa447ad82e4f89ca05bdc84f03e3dd4f9b7b676",
                Kind::JarRawV1 => "8ade38703f4e8699a52945e8b7b8dbb279de60ab501a71cf9729025aaeeacc80",
                Kind::JarClassV1 => "2f9f436e2477827eac1d17f0100c9d12d10b1498591791990886b725021aa5dc",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-eql/x-pack-eql-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "4f90443bd3cdca6adc77e4851ac33d38b3031128a55425094908301136ac50c2",
                Kind::RawSha256 => "cb26b61a44f2706cced9b5b66acae7187dfd4a164a2af98e84dee97b3cba0a6b",
                Kind::JarRawV1 => "57cc512020cbdc4483da867c27e8aae6ef93dd305e89c543eceb0d60c608150c",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-fleet/x-pack-fleet-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "cb51c29c3f78320ea48d6559d19c6e08eeeb9d2a0ddf865e2b536533a1d16f28",
                Kind::JarClassV1 => "0d8f92a222b5b0877e32fdc5199c1514b94373f22dd58b063fde45a1cab95bea",
                Kind::RawSha256 => "4688efa17c8212ef21ef47240d224a22f2442b75fb9f12e96f51dedf434eed6e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-graph/x-pack-graph-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "a424751fd933146df6d4a15dcd871352c90a320de8ff9ca97208cc6c26456a7f",
                Kind::JarRawV1 => "dd9e61c843e1844d48bc0f8523a476ba062e14e740f43914f8bf5698ca2e056c",
                Kind::JarClassV1 => "c68bc2f22ff07390e213dc94bc3c42587f56ba1f742653ecedf8e377863f32cc",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/cryptacular-1.2.4.jar",
            combined! {
                Kind::JarClassV1 => "cc49125e2781a6a2e6fa1b2202d2f3ec7879ec0df717421e4108dd24b5d97309",
                Kind::JarRawV1 => "e099b93f55bf83faab7149172198f914eabc25e42535bbc627ad0f4f2833a3c5",
                Kind::RawSha256 => "97feff80494a54f1b5001f6f4bbdbd45cb64ccbb2dffeb679da9da9be0434b07",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/guava-19.0.jar",
            combined! {
                Kind::JarRawV1 => "c44bee0c1835c70c899b3e67b16d1eb2961e291e614d6213bdfbb283e1332937",
                Kind::RawSha256 => "58d4cc2e05ebb012bbac568b032f75623be1cb6fb096f3c60c72a86f7f057de4",
                Kind::JarClassV1 => "95aee2084dbaf92ffcfa933d62f8a69a1af9a0db979356798ffe650f91d8414f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/httpclient-cache-4.5.10.jar",
            combined! {
                Kind::RawSha256 => "beff09f209510332d30f9444785a06755da2686d92338e37581cd62c5a2844bf",
                Kind::JarRawV1 => "ca79bb1086ec37e97617325b7de3b83f39252466a0bd6c86129a83e5d82546d4",
                Kind::JarClassV1 => "93642af3a14cd722355061aea13e107eacd474ebf17fca5db0111f24943f1c1a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/java-support-7.5.1.jar",
            combined! {
                Kind::RawSha256 => "391d97def8b84474176f4144012be7a67ed3f77ab4111ef41e4ed232f9e07b64",
                Kind::JarClassV1 => "cfd64420bfb541613084ebe96e501c125abde2c875fee8cc7c389705a6383101",
                Kind::JarRawV1 => "ae8aaf32556f051325cd97ec33e4a01ae46df0b44b17cca32ed706b4786a755b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/log4j-slf4j-impl-2.17.1.jar",
            combined! {
                Kind::JarRawV1 => "a78204ec2fbf25286ad7f7c7d497843d9758c49b55895c8a413998b74a10428e",
                Kind::RawSha256 => "e9a03720e5d5076009c2530635da9d08485e28a0b0ec20708dadc51afb78e41e",
                Kind::JarClassV1 => "9ad437ba29a9f5f245d4ccd6fc067aaa9a4e02dc7c677d61ffbc395738971862",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/metrics-core-3.2.2.jar",
            combined! {
                Kind::JarRawV1 => "e0bd824dcf66c22830cd59080b1ee7e7054046fc4fd47593193a90eca60cace9",
                Kind::RawSha256 => "5c6f685e41664d10c70c65837cba9e58d39ff3896811e3b5707a934b11c85ad0",
                Kind::JarClassV1 => "128a2a3a64c5bb2ac06d949b38774d0b6db58aeb330caaed07fb50b28711be78",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-core-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "30206cc29f4da57fdd128cfd01c1ac896ec11ae3ca1931928f643a32abcaebc0",
                Kind::RawSha256 => "da8c91082b618bd02bd6300c688b9336926ba49b6206b3fefb421dccee288182",
                Kind::JarRawV1 => "648d7df7af428052309e2a7b637e98ec9ae1f7db55f4cb4670cef0081d17e297",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-messaging-api-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "4daec9276f140b72d79e9ded314c07077bfb51d67d13207832edd282ef0d95eb",
                Kind::JarRawV1 => "545763f27b6878cbffb60b3216df562315d99b1d28fad2dfe14496046051a0e9",
                Kind::JarClassV1 => "b9b561c29bac7d7103458e87323b079696dae6f098307a6342f6cbaf6ed10395",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-messaging-impl-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "d275f9b50750aef585e3d0ce97513a142e4b2911ecafa44d977781de99bd1cf1",
                Kind::RawSha256 => "7cd6075a4590120d7863a83d6619c3f094776c6ebe542a33d48ca4901d70cb50",
                Kind::JarRawV1 => "c531a483b3e1fb4654e63a213a5fb07dacec89b0b84ae6a71d112ecb04808f32",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-profile-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "6b0ed5a5fda57c040492cb9d634a361e9d6439ff6aaedce52939e5f69be6ed20",
                Kind::JarRawV1 => "bdb3ba259b74280ee850e3e4f9d123190517a1a839e3ae55ef150a55c6313546",
                Kind::RawSha256 => "e04c83aa6115da14878c642d008586ff11c0263ad723a3153f7153806da9c5a0",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-profile-impl-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "578bd06971997cea753a407394d6a63166e7219df5e251afd341e7b0abdd2dd4",
                Kind::JarClassV1 => "ba5131ef4582ab0810e11ea7c1cad42e5bf903a533aecd19f795242951293b62",
                Kind::RawSha256 => "bc67c64652f060d86b64649ca0345dd0c9e49cc622bd6ed7637e41845cd2db33",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-saml-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "86f859d1e74adbba0e36981834b019e6fb516133b846e0a315b8cec032b83243",
                Kind::RawSha256 => "5a1f3ab432f5163df8fbf7d0083bab9801a183d7544ac60b7f4bbccc7f59bc18",
                Kind::JarRawV1 => "34430593937be9c9fc61cda99a8215ced2020ed044d15c9dccf0ef99f598d5ce",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-saml-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "4e53cdbb3e95664535ed0070164ee074ea164336ce6a31beb3d1356385253051",
                Kind::JarRawV1 => "540234ba1c0ae9b4150a5a06bbcf0108a4797a5704801c2db937b058b04f1ab9",
                Kind::JarClassV1 => "4fb31d9dbfdc8a72d36988302a64c8310806177083978c87fa98d6e76756e8da",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-security-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "d625acaff8578c41b676baa4d203af41b633331cd7d275bbb05f17ff660f0a56",
                Kind::JarClassV1 => "8c52fab5aa69ca0be3dad921d2ed8631252de45981589ba363cc6430e3cb214a",
                Kind::RawSha256 => "a764325cb1ed4d1c137202a07312ebe5cffc161012c7abd721a3a0e4470c3e96",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-security-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "e18bdf84f021b3737c53e2d27bb4853e944abb3365a792f710da1571df9832a0",
                Kind::JarClassV1 => "1de11e29cb2cc56cd314743446e72346dcc2f0142cec73242b9eb588e01fe747",
                Kind::JarRawV1 => "35b28d8b5be147355c012370bd55295a268d29afc7c91454064b655fc21867f2",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-soap-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "877ddbbbd5f832578308591e3835fe771cc75775d0c5cf8da8edba6ffd507c6e",
                Kind::JarClassV1 => "313dcb824a955877cc29b8a0045c4aec66b33192990c3e16591dc0805d760747",
                Kind::RawSha256 => "841427d8eecec98287f4fdc5fc9acc706db16692fddc1a2adbefc6fd50f7f627",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-soap-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "9e499b084fff148b8c068a16d40035a3ba76283eb2250a37fca45eabbc3cb2e4",
                Kind::JarClassV1 => "74db32205f0917798c4bf629dc181282ab8de9086440037185d6b0fea419041d",
                Kind::JarRawV1 => "19190db7c20a5a7a5addad86e0c2097d3eda8b1bc651b00646a63e52ab6ccdf2",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-storage-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "6d76e7f3725086e63a4f944eb369e5e64b32600e1c6a90051467317939f14ff0",
                Kind::RawSha256 => "5d64e2aa94e440413f063124e1bd5ea14008fa0611854c9ef508eadc57871998",
                Kind::JarClassV1 => "edec8d01835f81c4346420b7bf04a0590f856f3bd5cb987ea4a81d6ffd9ff9d5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-storage-impl-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "20bccf256554e0a6ddf153559a275319b00fb5088f9115c1ef3ad85324b7537e",
                Kind::RawSha256 => "d7cfbce5fb8e7b6c0fa9c690cb0ec801cfe63d51798795452df3177d5838cefc",
                Kind::JarClassV1 => "b7259ee454e65a560d3458d429b6dd815cebea01258e7208d81d64acae1538e0",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-xmlsec-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "52d22e95423f03400cf40b5c06cfbda7840eefa0d5d3c4d8358b5dd64e926bae",
                Kind::JarRawV1 => "a42ca8103feba47d644962e1102c07753fcf8f4f38191afb7e06d73c9a3d1c91",
                Kind::RawSha256 => "fa1b5971556e26b66a4a6dc119d17b6c1f99b3d3288ae36dfbaaf262330c03da",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/opensaml-xmlsec-impl-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "758e9b563673cb2fffe760102fe5f2a6d735810fe79cbaf2ef1bcfdf5f9298c8",
                Kind::JarRawV1 => "07470be684e0a25d384c89523ad9fca603d54c5927a335708303e42a905edc22",
                Kind::RawSha256 => "6bb4be41b1aa9e9f6a122504b77cc85a16f11427fb9ecd6d096bbaf0e5a882df",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/slf4j-api-1.6.2.jar",
            combined! {
                Kind::RawSha256 => "c26413df1741466d8bec549a4b7f6480acb2ac928b156be8c5e3feaf9cba1b59",
                Kind::JarRawV1 => "51bf440ef88e6f86d2bb6097c537b5afaf8ca93a7fe4a3ee1a203113a6f279d3",
                Kind::JarClassV1 => "af24a6f8c98dd2a81bfbfa9503452c31b64900b76e33a258e102086f661c1133",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/x-pack-identity-provider-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "a2735bfb8fc4a4c8e29f7dbdce281db54cbd30b60168cddd3d50bf2425c1c3be",
                Kind::JarClassV1 => "eece9bb28f366440830bfa484b151407375b3d5a6a30a78ad03bb74d4a9f9d53",
                Kind::RawSha256 => "367fe66628317c896740b163f8787b55e97a0edc402b1bd27b7946476f3f78e5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-identity-provider/xmlsec-2.1.4.jar",
            combined! {
                Kind::JarRawV1 => "f3d89c5f5cbbd574c6e0553e412760d6d0c0a9847e3a79b0c4d4df2b040f6e5c",
                Kind::JarClassV1 => "16fcca024d61653302b7ae4a4a3997eae7e93a7c6a34a8f6fe8bed60de02cff0",
                Kind::RawSha256 => "2e2ec8fe0cf873979f630ae4d35e7ede3390321279b7a15de9deed3f3430990c",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ilm/x-pack-ilm-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "870929b111d688ebc695478dea6811ab131ffd1b54da600f3d25eb661df2a93c",
                Kind::JarRawV1 => "8e2668ff2e6f43f234aa90a687ce046b69986b6e70a1d89934040266524f4814",
                Kind::JarClassV1 => "aaefa492d637c7eac6299f012a5833d06909e723aef7c069704fc795fa80ea17",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-logstash/x-pack-logstash-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "02e0fcf1707016cbf78a8c7b7a6e56750abb20d23f8b8dc86bd4a4af9a5596a6",
                Kind::RawSha256 => "b2e1b1d5f14b8306382846598d2a44bb76da918fe7fb451a1bec072ac4ed6696",
                Kind::JarRawV1 => "654b08487a7e66e3d2665d97401c3a5f4075791684a5211535a326460855508f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ml/commons-math3-3.6.1.jar",
            combined! {
                Kind::JarRawV1 => "de8a86e786ccdfae373551eee29b07428acdc7ac9eeb495f5b23fed90f36ff35",
                Kind::RawSha256 => "1e56d7b058d28b65abd256b8458e3885b674c1d588fa43cd7d1cbb9c7ef2b308",
                Kind::JarClassV1 => "6bf92ce1f74505c615e5224765586998b0f9aecfa2d790479cd668f28d7101dd",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ml/elasticsearch-grok-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "fee1097201de7908879f65cdaf0fc54bc60a4e023450ca1f9eaf3ffb7b14bc84",
                Kind::JarRawV1 => "3244f3a0b70d8cb7cf4c44e31806c14f038374b85b98dd965a9fee5ca7709d2e",
                Kind::RawSha256 => "2e524c254feecbb5d41b42d4a4d8eb3ed96f3d57b1e7c9fd0ef4839581c49dc1",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ml/jcodings-1.0.44.jar",
            combined! {
                Kind::JarRawV1 => "3ff823499e3d4fd4815efbf4a54311290f5a64592d944e901f8a2940009b35dd",
                Kind::JarClassV1 => "4dd715e0832222e95234af07d22995a0cc40fdc7328ec88b355877d060650532",
                Kind::RawSha256 => "49190d6ad09056de57d7ed41ed5b4b105e033557b5dd170702decdcf05ee341a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ml/joni-2.1.29.jar",
            combined! {
                Kind::JarRawV1 => "5f5c1894e9780c6e408de3baeee7585886d8fbc3ae73b6af25722912567cdcff",
                Kind::RawSha256 => "aa4b71415682f3d7fa44083cd94a9ec48478ec3b9c30947b4152913d41b1004d",
                Kind::JarClassV1 => "e9629b51e39e64cd10c5a7a7e10dbb0c5428ce4006a864da5fba4e1a1ec03ff2",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ml/super-csv-2.4.0.jar",
            combined! {
                Kind::JarRawV1 => "f7d9ca7b26a2e4495ac6f4b864d0749d2ab844c769ebc262149c37fd8cdb0287",
                Kind::RawSha256 => "cb3cc48f3cb521a6eb90b2984f98935dce4f184d43ff4aba052f4749a4131d4c",
                Kind::JarClassV1 => "1d57e9b8ab563b3c964935a3cee6aa96a4f3d4d83adddeb98a5076b6a10aee86",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ml/x-pack-ml-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "65d04770731a724c091585e9f6a8fc52adea4c3af4dbddf83ff9646428f173ce",
                Kind::RawSha256 => "b2593cc53b0a3a8ab2bd1bce2030356f95ff308a5b90d6d5f98b1b1f0551cdfa",
                Kind::JarClassV1 => "99fcfa9d6de3f4cb251f289c0849337cdc47bf9d55ae21327d32beb39af29975",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-monitoring/elasticsearch-rest-client-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
                Kind::RawSha256 => "d9c00a48a62e79354c871ffe3a45c18d6110b99d53424d9a15d8a370c70f091e",
                Kind::JarRawV1 => "63c408bfa682880f025860b8eeeb3aaf23abd79d50af3d06e10972b9bee3c4c8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-monitoring/elasticsearch-rest-client-sniffer-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "a1dd5002b52f0cc9b2f37176ce492036514df0e26fc693b0572a549d53969875",
                Kind::JarRawV1 => "02aef50dd3f51a2d2cb8ad8bd087f68fdbb2c9b0e5bb398a3cc154ec16c25043",
                Kind::RawSha256 => "572370053f371ce51a5ccc98e7960b98313a52b2b03d461830e3dcc289a95428",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-monitoring/x-pack-monitoring-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "b141c17e941c95d6820829e9854ae07b093907862e425980bdefae57894fda47",
                Kind::RawSha256 => "a49f26ce4dc6c52556d7705e9305e9826593a9fd34bcecf38ea0e65daef63452",
                Kind::JarRawV1 => "505c91488b9c7f0f05a6568120f4c107d54d3deac1a9a782d21a2c2e337f29b3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ql/antlr4-runtime-4.9.2.jar",
            combined! {
                Kind::RawSha256 => "120053628dd598d43cb7ac6b9ecc72529dfa5a5fd3292d37cf638a81cc0075f6",
                Kind::JarRawV1 => "751394f157b9a82d14cf82bdc1d9c84406ae49c51baa7cc3b2e0b6c4eaef36bc",
                Kind::JarClassV1 => "06b44f4b2db1cb279833f864d2f955d9015337e2278406269cf5070fc30aaca3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-ql/x-pack-ql-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "544217468bae50f757e557156ef8fc06c192833af33965dd2d27fa5c3b8cf569",
                Kind::JarRawV1 => "792223fd5495dc33717053a0fadd216d04321d26f615c7c1ea777a1f2d484e61",
                Kind::RawSha256 => "ef1a834ba959912502e23f4a45fec615558f0e8d8c4474812a987ec4ebd531e9",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-rollup/x-pack-rollup-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "c42df93fbb23310c34e90ab815ee79e59331b5551c4acb75b3d89ae18c6cacd9",
                Kind::RawSha256 => "b460500ccb3366975acda918075c41270a5d4b72b5810c562de3b54904498992",
                Kind::JarRawV1 => "2721038124e5296cbb62f3a78f7fdde93ee5e946b09148cd590c5839659a142f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/accessors-smart-2.4.2.jar",
            combined! {
                Kind::JarRawV1 => "6c5a85ffdbdfd1cbe3ebf9112f3aa6d5af4e2b58b040743d1ea4b08a6df98d8e",
                Kind::JarClassV1 => "70e28536d92a7e3063561f9eec510155d1051e450a264890571ffc5b1ba3c14d",
                Kind::RawSha256 => "0972bbc99437c4163acd09b630e6c77eab4cfab8a9594621c95466c0c6645396",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/asm-9.1.jar",
            combined! {
                Kind::JarRawV1 => "d7e561bcafa78354f3934bbb4365f6f84236f02e2c0e2d9291ce8b2ffaf57f2f",
                Kind::JarClassV1 => "946cd8d0c6cef02fe3b4f475ea5acc52d9c761a702baa53f6b10ae4476c9d296",
                Kind::RawSha256 => "cda4de455fab48ff0bcb7c48b4639447d4de859a7afc30a094a986f0936beba2",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/cryptacular-1.2.4.jar",
            combined! {
                Kind::RawSha256 => "97feff80494a54f1b5001f6f4bbdbd45cb64ccbb2dffeb679da9da9be0434b07",
                Kind::JarClassV1 => "cc49125e2781a6a2e6fa1b2202d2f3ec7879ec0df717421e4108dd24b5d97309",
                Kind::JarRawV1 => "e099b93f55bf83faab7149172198f914eabc25e42535bbc627ad0f4f2833a3c5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/guava-19.0.jar",
            combined! {
                Kind::JarRawV1 => "c44bee0c1835c70c899b3e67b16d1eb2961e291e614d6213bdfbb283e1332937",
                Kind::RawSha256 => "58d4cc2e05ebb012bbac568b032f75623be1cb6fb096f3c60c72a86f7f057de4",
                Kind::JarClassV1 => "95aee2084dbaf92ffcfa933d62f8a69a1af9a0db979356798ffe650f91d8414f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/httpclient-cache-4.5.10.jar",
            combined! {
                Kind::JarRawV1 => "ca79bb1086ec37e97617325b7de3b83f39252466a0bd6c86129a83e5d82546d4",
                Kind::RawSha256 => "beff09f209510332d30f9444785a06755da2686d92338e37581cd62c5a2844bf",
                Kind::JarClassV1 => "93642af3a14cd722355061aea13e107eacd474ebf17fca5db0111f24943f1c1a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/jakarta.mail-1.6.3.jar",
            combined! {
                Kind::RawSha256 => "018ffd5684fd758157886933cc74116996d7f5757cc6f104bb43a647a3815f8a",
                Kind::JarRawV1 => "2167df3dfa9546ff745eae83c51e1cb25cd9c33c490e80a496a9582d829d5196",
                Kind::JarClassV1 => "bbad1626bc0a6b85c0900559032ce300b47c6dea0cf10d0f0e0eb0e73419f5c3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/java-support-7.5.1.jar",
            combined! {
                Kind::JarClassV1 => "cfd64420bfb541613084ebe96e501c125abde2c875fee8cc7c389705a6383101",
                Kind::RawSha256 => "391d97def8b84474176f4144012be7a67ed3f77ab4111ef41e4ed232f9e07b64",
                Kind::JarRawV1 => "ae8aaf32556f051325cd97ec33e4a01ae46df0b44b17cca32ed706b4786a755b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/jcip-annotations-1.0.jar",
            combined! {
                Kind::JarClassV1 => "0fb33f79073ee3e1c21fd5007e20f8834b702cfcd01004e478609b35d4a122e2",
                Kind::RawSha256 => "be5805392060c71474bf6c9a67a099471274d30b83eef84bfc4e0889a4f1dcc0",
                Kind::JarRawV1 => "c93da171e66febbd2b43baeb1d705b15055e71fc76b23b86f6713e2d613c668a",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/json-smart-2.4.2.jar",
            combined! {
                Kind::RawSha256 => "64072f56d9dff5040b2acec477c5d5e6bcebfc88c508f12acb26072d07942146",
                Kind::JarClassV1 => "6b0caf174209850c97ce18338d4675a7a16cd5db87216f4ea7969a86ad2ca2ed",
                Kind::JarRawV1 => "071928fda8ed48a1cd47e67216939d5657eb67ee9c20510a713af206abc2559f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/lang-tag-1.4.4.jar",
            combined! {
                Kind::JarClassV1 => "204546610f8e9e399c584e89bd20834fa03bc6d86046325919aeee15e3c98fc9",
                Kind::JarRawV1 => "71897cfddeecc474bb9ffb52555edd5372272ece6bbc153d9b49e512d1985dc9",
                Kind::RawSha256 => "e49d2c694bb80c7036c177f2aabf53b7156061a68bd19dfd60e2bd370709e0c5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/log4j-slf4j-impl-2.17.1.jar",
            combined! {
                Kind::RawSha256 => "e9a03720e5d5076009c2530635da9d08485e28a0b0ec20708dadc51afb78e41e",
                Kind::JarRawV1 => "a78204ec2fbf25286ad7f7c7d497843d9758c49b55895c8a413998b74a10428e",
                Kind::JarClassV1 => "9ad437ba29a9f5f245d4ccd6fc067aaa9a4e02dc7c677d61ffbc395738971862",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/metrics-core-3.2.2.jar",
            combined! {
                Kind::RawSha256 => "5c6f685e41664d10c70c65837cba9e58d39ff3896811e3b5707a934b11c85ad0",
                Kind::JarRawV1 => "e0bd824dcf66c22830cd59080b1ee7e7054046fc4fd47593193a90eca60cace9",
                Kind::JarClassV1 => "128a2a3a64c5bb2ac06d949b38774d0b6db58aeb330caaed07fb50b28711be78",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/nimbus-jose-jwt-9.8.1.jar",
            combined! {
                Kind::JarRawV1 => "73b812dff00d124f9e8ae03d47099342c3943624d0b2c5afcca91c712e6367e3",
                Kind::RawSha256 => "7664cf8c6f2adadf600287812b32878277beda54912eab9d4c2932cd50cb704a",
                Kind::JarClassV1 => "2b01ebf997a0e6f43579712c4f7dd6204dfd3f533192ee1430e0a2ccc7b9adb4",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/oauth2-oidc-sdk-9.3.1.jar",
            combined! {
                Kind::JarClassV1 => "a060fa10d2a1596231ce56651cefe35c0d37d82b0fbff4b46472c5363a02d856",
                Kind::RawSha256 => "6ebf20268af33b31fc21486a59571b03013940a5e051cb4925d08c2c1e6e017c",
                Kind::JarRawV1 => "95aa29e31b02fec3e0c2a905033d674dbc10a35cbd5eb51c97b8a0e978f8783c",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-core-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "30206cc29f4da57fdd128cfd01c1ac896ec11ae3ca1931928f643a32abcaebc0",
                Kind::RawSha256 => "da8c91082b618bd02bd6300c688b9336926ba49b6206b3fefb421dccee288182",
                Kind::JarRawV1 => "648d7df7af428052309e2a7b637e98ec9ae1f7db55f4cb4670cef0081d17e297",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-messaging-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "545763f27b6878cbffb60b3216df562315d99b1d28fad2dfe14496046051a0e9",
                Kind::JarClassV1 => "b9b561c29bac7d7103458e87323b079696dae6f098307a6342f6cbaf6ed10395",
                Kind::RawSha256 => "4daec9276f140b72d79e9ded314c07077bfb51d67d13207832edd282ef0d95eb",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-messaging-impl-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "d275f9b50750aef585e3d0ce97513a142e4b2911ecafa44d977781de99bd1cf1",
                Kind::RawSha256 => "7cd6075a4590120d7863a83d6619c3f094776c6ebe542a33d48ca4901d70cb50",
                Kind::JarRawV1 => "c531a483b3e1fb4654e63a213a5fb07dacec89b0b84ae6a71d112ecb04808f32",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-profile-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "6b0ed5a5fda57c040492cb9d634a361e9d6439ff6aaedce52939e5f69be6ed20",
                Kind::RawSha256 => "e04c83aa6115da14878c642d008586ff11c0263ad723a3153f7153806da9c5a0",
                Kind::JarRawV1 => "bdb3ba259b74280ee850e3e4f9d123190517a1a839e3ae55ef150a55c6313546",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-profile-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "bc67c64652f060d86b64649ca0345dd0c9e49cc622bd6ed7637e41845cd2db33",
                Kind::JarRawV1 => "578bd06971997cea753a407394d6a63166e7219df5e251afd341e7b0abdd2dd4",
                Kind::JarClassV1 => "ba5131ef4582ab0810e11ea7c1cad42e5bf903a533aecd19f795242951293b62",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-saml-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "86f859d1e74adbba0e36981834b019e6fb516133b846e0a315b8cec032b83243",
                Kind::RawSha256 => "5a1f3ab432f5163df8fbf7d0083bab9801a183d7544ac60b7f4bbccc7f59bc18",
                Kind::JarRawV1 => "34430593937be9c9fc61cda99a8215ced2020ed044d15c9dccf0ef99f598d5ce",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-saml-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "4e53cdbb3e95664535ed0070164ee074ea164336ce6a31beb3d1356385253051",
                Kind::JarRawV1 => "540234ba1c0ae9b4150a5a06bbcf0108a4797a5704801c2db937b058b04f1ab9",
                Kind::JarClassV1 => "4fb31d9dbfdc8a72d36988302a64c8310806177083978c87fa98d6e76756e8da",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-security-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "8c52fab5aa69ca0be3dad921d2ed8631252de45981589ba363cc6430e3cb214a",
                Kind::JarRawV1 => "d625acaff8578c41b676baa4d203af41b633331cd7d275bbb05f17ff660f0a56",
                Kind::RawSha256 => "a764325cb1ed4d1c137202a07312ebe5cffc161012c7abd721a3a0e4470c3e96",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-security-impl-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "1de11e29cb2cc56cd314743446e72346dcc2f0142cec73242b9eb588e01fe747",
                Kind::JarRawV1 => "35b28d8b5be147355c012370bd55295a268d29afc7c91454064b655fc21867f2",
                Kind::RawSha256 => "e18bdf84f021b3737c53e2d27bb4853e944abb3365a792f710da1571df9832a0",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-soap-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "313dcb824a955877cc29b8a0045c4aec66b33192990c3e16591dc0805d760747",
                Kind::RawSha256 => "841427d8eecec98287f4fdc5fc9acc706db16692fddc1a2adbefc6fd50f7f627",
                Kind::JarRawV1 => "877ddbbbd5f832578308591e3835fe771cc75775d0c5cf8da8edba6ffd507c6e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-soap-impl-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "19190db7c20a5a7a5addad86e0c2097d3eda8b1bc651b00646a63e52ab6ccdf2",
                Kind::RawSha256 => "9e499b084fff148b8c068a16d40035a3ba76283eb2250a37fca45eabbc3cb2e4",
                Kind::JarClassV1 => "74db32205f0917798c4bf629dc181282ab8de9086440037185d6b0fea419041d",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-storage-api-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "5d64e2aa94e440413f063124e1bd5ea14008fa0611854c9ef508eadc57871998",
                Kind::JarRawV1 => "6d76e7f3725086e63a4f944eb369e5e64b32600e1c6a90051467317939f14ff0",
                Kind::JarClassV1 => "edec8d01835f81c4346420b7bf04a0590f856f3bd5cb987ea4a81d6ffd9ff9d5",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-storage-impl-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "20bccf256554e0a6ddf153559a275319b00fb5088f9115c1ef3ad85324b7537e",
                Kind::JarClassV1 => "b7259ee454e65a560d3458d429b6dd815cebea01258e7208d81d64acae1538e0",
                Kind::RawSha256 => "d7cfbce5fb8e7b6c0fa9c690cb0ec801cfe63d51798795452df3177d5838cefc",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-xmlsec-api-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "fa1b5971556e26b66a4a6dc119d17b6c1f99b3d3288ae36dfbaaf262330c03da",
                Kind::JarRawV1 => "a42ca8103feba47d644962e1102c07753fcf8f4f38191afb7e06d73c9a3d1c91",
                Kind::JarClassV1 => "52d22e95423f03400cf40b5c06cfbda7840eefa0d5d3c4d8358b5dd64e926bae",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/opensaml-xmlsec-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "6bb4be41b1aa9e9f6a122504b77cc85a16f11427fb9ecd6d096bbaf0e5a882df",
                Kind::JarRawV1 => "07470be684e0a25d384c89523ad9fca603d54c5927a335708303e42a905edc22",
                Kind::JarClassV1 => "758e9b563673cb2fffe760102fe5f2a6d735810fe79cbaf2ef1bcfdf5f9298c8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/slf4j-api-1.6.2.jar",
            combined! {
                Kind::JarRawV1 => "51bf440ef88e6f86d2bb6097c537b5afaf8ca93a7fe4a3ee1a203113a6f279d3",
                Kind::RawSha256 => "c26413df1741466d8bec549a4b7f6480acb2ac928b156be8c5e3feaf9cba1b59",
                Kind::JarClassV1 => "af24a6f8c98dd2a81bfbfa9503452c31b64900b76e33a258e102086f661c1133",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/x-pack-security-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "f3ad5adb256661f28569fdaa2f23416d7e29175b97f19b9d7fa0097fe15a1e8f",
                Kind::RawSha256 => "7a7e03817869809b0e68cdfc60e4865b639fa3e381e818ac3ed9dea6d8c8ed73",
                Kind::JarRawV1 => "cabc1a9078ccc5a78adb07d33bd6ea2a3592e8e8efa06b21c797e047cdf39205",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-security/xmlsec-2.1.4.jar",
            combined! {
                Kind::JarClassV1 => "16fcca024d61653302b7ae4a4a3997eae7e93a7c6a34a8f6fe8bed60de02cff0",
                Kind::RawSha256 => "2e2ec8fe0cf873979f630ae4d35e7ede3390321279b7a15de9deed3f3430990c",
                Kind::JarRawV1 => "f3d89c5f5cbbd574c6e0553e412760d6d0c0a9847e3a79b0c4d4df2b040f6e5c",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-shutdown/x-pack-shutdown-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "82137aa51b085f1dbdf42b6d9e49994652acca0759d1316a1289b4275f27e656",
                Kind::JarClassV1 => "de426421e4ab9e71749a1f390a088890c13fdac9e9d63cb7fede958faa88f01d",
                Kind::JarRawV1 => "d2bc6c3669282397ab005b6195144a636870559fc4a82761589cabbaf893aec0",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-sql/aggs-matrix-stats-client-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "aa2a5bcac58731f1c102036b5030ba2b520728591696dde4dc45c6623127fdea",
                Kind::RawSha256 => "37633e596eb108f031abf99f743de6035efcde70f8a8973e72ceffc724d94d02",
                Kind::JarRawV1 => "3cf096f9b1cb2e9b047c0a65688207cd57cce1fbcf60875a73baabac6b173924",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-sql/sql-action-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "3f2cc8806776c4204f9e7b18b8713e298698719f8dc9e72093a9adfc7438677a",
                Kind::JarRawV1 => "a4e7c276ee1ed37c26d9d8eff2fdf500117a256a7b66fa05bea59cedea3fdffc",
                Kind::JarClassV1 => "7a8488a0813c0bdba58d548aca86ca409950136ef9cd980154aa4883bd1bd30b",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-sql/sql-proto-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "aff589b704d0a56889b2b48ad9df5ef63774abbc2478b26619e1e0aee27ad061",
                Kind::JarClassV1 => "9ef631ca6ad292b6041189ea822b1ffcbc706ae6bd1e0425276f6a30dc90d88b",
                Kind::RawSha256 => "ab0d99b4cf3184ac64d498d9ed8f89dc1c134fd622577a89efd1450587730f62",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-sql/x-pack-sql-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "c1ba6344f8f90f5481a957f1fc90027f8d75564e1eea3d19acce72887f3f74db",
                Kind::JarRawV1 => "efdd3f0769d209c52dc68118d125a9d1d0da8a42be7715703da159c2cd727022",
                Kind::JarClassV1 => "fcda9fafedc9d0a8a2715da4ffd02350090c92df8ac546178959801d399ed16c",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-stack/x-pack-stack-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "1da0b0991d6f68e8503cc209c2681ac12ee9887e550dab75e00e720505fc15c1",
                Kind::RawSha256 => "3865e5d1a9f55830137b687219e4299fbe2a8cf5cecd22c0d8f004c2b4a7f27e",
                Kind::JarClassV1 => "2f2d33886dd94ab87918d7d03b8acb3e5683bd8c3b4346c0a825a092cb5de014",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-text-structure/elasticsearch-grok-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "2e524c254feecbb5d41b42d4a4d8eb3ed96f3d57b1e7c9fd0ef4839581c49dc1",
                Kind::JarClassV1 => "fee1097201de7908879f65cdaf0fc54bc60a4e023450ca1f9eaf3ffb7b14bc84",
                Kind::JarRawV1 => "3244f3a0b70d8cb7cf4c44e31806c14f038374b85b98dd965a9fee5ca7709d2e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-text-structure/icu4j-62.1.jar",
            combined! {
                Kind::JarRawV1 => "280e4a3a79a5ee084ba59e4fdbe4c59582ea2e56180a5ead031a56cd34f04e36",
                Kind::JarClassV1 => "ed02dfc3dd7375901fdb24aa1fbdf17da49eed182f38d34bf5c8eccc1c53e16e",
                Kind::RawSha256 => "e5b0962e566003d68bf3ec7d87b60e279f5cc7aa93cbc4056432803046512478",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-text-structure/jcodings-1.0.44.jar",
            combined! {
                Kind::JarClassV1 => "4dd715e0832222e95234af07d22995a0cc40fdc7328ec88b355877d060650532",
                Kind::RawSha256 => "49190d6ad09056de57d7ed41ed5b4b105e033557b5dd170702decdcf05ee341a",
                Kind::JarRawV1 => "3ff823499e3d4fd4815efbf4a54311290f5a64592d944e901f8a2940009b35dd",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-text-structure/joni-2.1.29.jar",
            combined! {
                Kind::RawSha256 => "aa4b71415682f3d7fa44083cd94a9ec48478ec3b9c30947b4152913d41b1004d",
                Kind::JarClassV1 => "e9629b51e39e64cd10c5a7a7e10dbb0c5428ce4006a864da5fba4e1a1ec03ff2",
                Kind::JarRawV1 => "5f5c1894e9780c6e408de3baeee7585886d8fbc3ae73b6af25722912567cdcff",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-text-structure/super-csv-2.4.0.jar",
            combined! {
                Kind::JarClassV1 => "1d57e9b8ab563b3c964935a3cee6aa96a4f3d4d83adddeb98a5076b6a10aee86",
                Kind::JarRawV1 => "f7d9ca7b26a2e4495ac6f4b864d0749d2ab844c769ebc262149c37fd8cdb0287",
                Kind::RawSha256 => "cb3cc48f3cb521a6eb90b2984f98935dce4f184d43ff4aba052f4749a4131d4c",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-text-structure/x-pack-text-structure-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "5fcaee73c8a7e1633c4beb8aae477a26e982f972d00ae3fa7debec1f920f029d",
                Kind::RawSha256 => "bc7d8a1729568625a2c93725ca5b693266621c61b6be9917645ce98cf8cd6086",
                Kind::JarRawV1 => "de69c1d1d1f20acc42c24cb5c65433962da5ab7411902a3b1e731d99aebf7370",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-voting-only-node/x-pack-voting-only-node-7.17.7.jar",
            combined! {
                Kind::JarClassV1 => "942a64c517ea90df3b87c079e3661ab2250374d12639eba72fc76abd51a7f3c9",
                Kind::RawSha256 => "2c574117180df6835597c0d6ca8bcb905445df66471058f329df83b83d9201ac",
                Kind::JarRawV1 => "2c18325219706ae4adc8df041eafed5439efed862c38bfdee46b7b1afb04e665",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-watcher/failureaccess-1.0.1.jar",
            combined! {
                Kind::RawSha256 => "a171ee4c734dd2da837e4b16be9df4661afab72a41adaf31eb84dfdaf936ca26",
                Kind::JarClassV1 => "98aebe7db02141ace8153c2e3dda72bf4657ce4623fbfea2e58e2b16560eb16e",
                Kind::JarRawV1 => "31a953705a18c0d8c4a90955d8b2e21e5ed43f0301e2c54728ff10d0bed33d94",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-watcher/guava-27.1-jre.jar",
            combined! {
                Kind::JarRawV1 => "35452fc5d9c74c5c8a0db9afaef386438a6b0df2987dd8770a1a4ea2655fbe02",
                Kind::JarClassV1 => "46f5ffb045cac212e986f59718a8540ae7b99251f6f866caedb15ca27d6f1071",
                Kind::RawSha256 => "4a5aa70cc968a4d137e599ad37553e5cfeed2265e8c193476d7119036c536fe7",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-watcher/jakarta.activation-1.2.1.jar",
            combined! {
                Kind::JarClassV1 => "94fe3153c5d00d7c5dda0e155d0ea7a24b2766f2a04e3fd7002212f28443f8d2",
                Kind::RawSha256 => "d84d4ba8b55cdb7fdcbb885e6939386367433f56f5ab8cfdc302a7c3587fa92b",
                Kind::JarRawV1 => "d2bf36ebf70edac4242ed5f0650a0b6ff1a883fa7c5a1abf06be62b1457afca6",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-watcher/jakarta.mail-1.6.4.jar",
            combined! {
                Kind::RawSha256 => "65d4c18e15ea2b9eb129098ae92db4cf996d85179f30ac34f7d4db856ffaa3f9",
                Kind::JarRawV1 => "be0291c7ac8dcbbb240c676ee0bf2f616f29d5deeb21b5d8e8df7784e87d5ce3",
                Kind::JarClassV1 => "832c3daba0fc677319db6860894ac2eee66422ec80ed3b56e6dcaa053bff766f",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-watcher/owasp-java-html-sanitizer-20211018.2.jar",
            combined! {
                Kind::RawSha256 => "48234cd74e35d91a31a683820a35b5b6d11b55527f32a5b162c6757408b95d7a",
                Kind::JarClassV1 => "ee1e9b0f2c6686c51c760e4bb0ad1374686067e4836ab011b2d3abe9826c87ee",
                Kind::JarRawV1 => "1af502f5a9e80a2a3014556476e3c3c2ad54f88df3a415218b9d2477a31ae5c7",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/modules/x-pack-watcher/x-pack-watcher-7.17.7.jar",
            combined! {
                Kind::JarRawV1 => "cd3c0f148286e75ea9c4eaa5581875492e5ac8d8a42fa1280a73ce76fc0e0f2b",
                Kind::RawSha256 => "3a29ee7b24fb7bdd6f50d43a0474ef383b622b166b4140b77df16c4bb452bc09",
                Kind::JarClassV1 => "ac8c8d84210f4f6f1348f6303f3025e00cfc69e5ba52ed8200341a4bc94703d3",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/aws-java-sdk-core-1.11.749.jar",
            combined! {
                Kind::JarClassV1 => "8dfbb2b4aaf46e526312d4f4f823930cfbc43f0b2c934706588fd85a14dcbe4e",
                Kind::RawSha256 => "06d4035a0b2373f0fef529de0e6f81ff43ef7a36ecf476e5d5f1aecba8ee4fad",
                Kind::JarRawV1 => "db3104afaa20a1accbc458e23c53aabc120a2fee1a75911402b9cffb0598ab82",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/aws-java-sdk-s3-1.11.749.jar",
            combined! {
                Kind::JarRawV1 => "c44602274b6d444cbdb9a9ae61580d285852aa002aeadc0e59a1c5cff615e05a",
                Kind::RawSha256 => "fc17c0e186c36ac153e9f871a510e485527c911edeb9f736160b37609f51e7c2",
                Kind::JarClassV1 => "1edb4b3f9b1847b5ab6b8d8d32033c924965d81d05a6792bf50ab938976ba713",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/commons-codec-1.11.jar",
            combined! {
                Kind::JarRawV1 => "592746b12d517d74550cd9056e12e0d1dd36ede46a87b29a61fc576f7c6abf2d",
                Kind::JarClassV1 => "e7e789b464c215ef83656198d719851f62b4a198f5a3aa03844c4c1f30aa1bce",
                Kind::RawSha256 => "e599d5318e97aa48f42136a2927e6dfa4e8881dff0e6c8e3109ddbbff51d7b7d",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/commons-logging-1.1.3.jar",
            combined! {
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/httpclient-4.5.10.jar",
            combined! {
                Kind::JarClassV1 => "444d77f0054d951ffeec24f16be3aef0ffc76edc546c7970fe4925aa3b71252b",
                Kind::RawSha256 => "38b9f16f504928e4db736a433b9cd10968d9ec8d6f5d0e61a64889a689172134",
                Kind::JarRawV1 => "8491995343994e75696217d4f400d2390942dac99c1e3c4a67532f9b9e71c846",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/httpcore-4.4.12.jar",
            combined! {
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/jackson-annotations-2.10.4.jar",
            combined! {
                Kind::JarRawV1 => "c2b037f66af48846c31348472e7028a9ec9c77c0deb1e6040f3b063cb555fd2b",
                Kind::RawSha256 => "9d07ea7ce579a678e7aea61249fa82c46469af9d02c5b5f13e84beb7b0827dbc",
                Kind::JarClassV1 => "a279dee3f1a0902b3fa6105457e11e62024399479dcc6a100657dfeda456f440",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/jackson-databind-2.10.4.jar",
            combined! {
                Kind::JarRawV1 => "5c7a86c671c8ffbddf5c4c9e7d40d38a345f1e8a329eee6d698e29744924a19e",
                Kind::JarClassV1 => "e3152904733e8aed7aff00033d6985c7a40dcc7b4d83b6ad04406431fe82431a",
                Kind::RawSha256 => "55312662a420c71508e6159c86aa41c1694c52e89a1b90dc94bcf4358134005e",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/jaxb-api-2.2.2.jar",
            combined! {
                Kind::RawSha256 => "30233df6215fb982d8784de91d307596748cea98d6d502293c7c3e85c1697137",
                Kind::JarRawV1 => "0bbe5c45762c9be8c9bf1e773bfe6c7ce91bd74e781cb04563019928f4a879c7",
                Kind::JarClassV1 => "0c56333d7c7d01c020a16370ba91215da83b46ddef942ffb4e3786c1946f1fe2",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/jmespath-java-1.11.749.jar",
            combined! {
                Kind::RawSha256 => "0a73428ee8e19b0a1cb4a39f5b4a7af8f5c2d736bd6b171d7054d517cd528c3e",
                Kind::JarRawV1 => "f7f5a59433d3a6ab6b0838d0f2add783e2422b585297d50aac234e0602ebb738",
                Kind::JarClassV1 => "eb6195033b2b1be63a7631a47ae0c7eebdaf668f47b66c720be2aafb5fa1caf8",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/log4j-1.2-api-2.17.1.jar",
            combined! {
                Kind::RawSha256 => "ca3e9150f95c31d15b9680a609b8817f8549bd395591c5ca55957d1ef0f464d6",
                Kind::JarRawV1 => "b6fa23d8eb7cad6b08c18f318a869f85778b0cdc9e9eac56a9fde7db263fdd72",
                Kind::JarClassV1 => "6e705c0891b1027df1444ecf13cce77d48cfa4b039bf2fbb596937a2f41bdfc1",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/elasticsearch/plugins/repository-s3/repository-s3-7.17.7.jar",
            combined! {
                Kind::RawSha256 => "419d9b0f8515fe1b3cb191aa6ceb361cdae2c41cda982fb754e9c90d57b73d6e",
                Kind::JarRawV1 => "f097c819aa04b4d147e1971998c24122185a1320898f36928512e11d93cdfcac",
                Kind::JarClassV1 => "f53c49ce4b4c6c2a9cf7952c06d7222f495521915a9946d47fa49e41266a4309",
            },
        ),
        DiscoveredJar::new(
            "opt/bitnami/java/lib/jrt-fs.jar",
            combined! {
                Kind::JarRawV1 => "eace54000eb4e70f0da69d8800a13fde22203be21e95f84d57969429af99e0ba",
                Kind::JarClassV1 => "1b9d4816dad40a7922d991be24f0f1bc4bffaf271ab45b9ba32f123a932dd807",
                Kind::RawSha256 => "808c30fd24ff3be4587464e804b4a1ab9a7b5b3a94fb9bd4cbe4bb28a9cb0f49",
            },
        ),
        ]
}
