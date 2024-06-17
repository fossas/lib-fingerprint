use fingerprint::Kind;

use crate::jar::DiscoveredJar;

use super::combined;

pub fn list() -> Vec<DiscoveredJar> {
    vec![
        DiscoveredJar::new(
            "usr/share/elasticsearch/bin/elasticsearch-sql-cli-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "e0f340bb10a632d221f7ceb0d1ca3a8d23203ef428e14d2ebe037b00252f68a5",
                Kind::JarClassV1 => "c21995af9737437c4111a6054c8bc2a6d7ac00c90b788b0b6d7b2489ec45973c",
                Kind::JarMavenCentralV1 => "af77644ebe1ae1f7ea048bfe5e36d3ca20739591",
                Kind::RawSha256 => "8a8da0f4cbac56bb4c79c49ffae1e30184a46d4495faf701b9ceadaf2a2684e3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/jdk/lib/jrt-fs.jar",
            combined! {
               Kind::JarRawV1 => "fda6923b9f32df9a02ad83699ceefbdafb408abfa3c8c917592653f53028db4c",
               Kind::JarClassV1 => "4f8c07e194059d85baa16a6413543afb88f47530df089231a18f938c802b9c3b",
               Kind::JarMavenCentralV1 => "e2260199d8b7d3e68eb13598ffab7b01a1f774e0",
               Kind::RawSha256 => "487c5a8ff1bcc6d27a2211fea000a3650804ca9e70ced54c086c0114fdd165dc",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/HdrHistogram-2.1.9.jar",
            combined! {
                Kind::JarClassV1 => "36369d225d43ba9449746c5151bc56030d741ec5069a79bba516383a78e0d93d",
                Kind::JarMavenCentralV1 => "e4631ce165eb400edecfa32e03d3f1be53dee754",
                Kind::RawSha256 => "95d40913be28dfd439cefea9170c40898ea84f11f25e6ff8de50339b8a7b5e3e",
                Kind::JarRawV1 => "9158db0485c6b3455e379e563e925ce03f4b5d81e62bf10ece305b8d14c30e0e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "dd46c52be9f9d9f619fda1b004c64343ef3bce922a7250d6c8e8b29316d63bb8",
                Kind::JarClassV1 => "54a43448cb33722739d5353e7b2dedc06a851c6f19a30df390a8c5755ed99adc",
                Kind::JarMavenCentralV1 => "845ffc75cc43e15fa6ce8039c71eb26aec0ebe64",
                Kind::RawSha256 => "820593396e46565776f96eef418829b1689afe92577b96f350dee4c22dc5ffc8",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-cli-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "62eb697375fa4ab5cab180d212a46e62cf710f7a0832f99bcc9831b88dd56259",
                Kind::JarMavenCentralV1 => "0644b8e8e6d36204c0c00402e090d724e2dd0608",
                Kind::JarRawV1 => "c76a476006f70ff7e9927be022a25e92f99feb9f64e251686cb846311b3350ab",
                Kind::RawSha256 => "8a03845fe20317d0e987a1d0deec544e938fb717c881921e991ae0546d2268ac",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-core-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "cc6ab596238354a915542ee0fa1e7869d13a84dfbb9b88f3e8317da7d8451bf7",
                Kind::JarRawV1 => "0f2d7ade5f486ee4e82bd0273f648d5f2a7d75961b43f40d49d5648473a4600d",
                Kind::JarClassV1 => "c0b6a3f3e8a1f005b75c64c8725add97837a07669c56124bc17469e6f4fdaae3",
                Kind::JarMavenCentralV1 => "b9bd1d86e7e1639bb349d37358fb8827570855f8",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-geo-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "331bcd766ecd88403efebec0cb4d2c130ca4d06e16f027c5a6be877511abe79a",
                Kind::JarMavenCentralV1 => "1c1428fb95141a26a468a2b6ccc49345ad4bf5f7",
                Kind::RawSha256 => "efb515da341bf0c1df363f09a88164d93a45242e87cfe19ceb9b39f446aa0ea7",
                Kind::JarRawV1 => "6f4f1ed8e0601b602e529102665ab346a837625ba69e6eaeed164198d0ec5956",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-launchers-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "3861758cd4c43a8391310e0a5caac44acf8730376eefda433988dfc41bcd4843",
                Kind::JarMavenCentralV1 => "88c8665bcbbac953cca335c68c50ce040f0c21a5",
                Kind::JarRawV1 => "3db206d04c604d3173058c5d011b32fd1f582a093ea7744a774b916c9f477f57",
                Kind::RawSha256 => "31043a80dcb389d175474eb2e4af88378745096ef04b4455782636315af8a5e6",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-log4j-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "601228abcb2f4a2ecab2f1da971fb99577061ef6128ed6eb79920c09cb20656f",
                Kind::JarClassV1 => "fbc4adb18d1d3b49b639491d51d333fdb43395d0fdbb1d7ecbc960f981f3a005",
                Kind::JarMavenCentralV1 => "35c5ff84f52336cbfce55b32baa573b38b9f45c0",
                Kind::JarRawV1 => "5452acf8354a640b29a1625a2de876891e5677914bd9d04842e17f10a03caa73",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-lz4-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "275e886a1d13bde2030d6b16baf1bf4454bbcafe92f878850e9d0d8f4b481c67",
                Kind::JarRawV1 => "3453ad8b73feadfe65f298582ef4683041367bc2143cc8ec4e86717716ad9df9",
                Kind::JarClassV1 => "e5044e0168e40068300126a309a55277b97f47964c0d737e77961311fa0f15f1",
                Kind::JarMavenCentralV1 => "b465c6e578e8afa49d9eda695a49e5542c77858a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-plugin-classloader-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "d57f4ebb3e6a9f266936e470e5af037de335074881e5c95d94b4b5731961dece",
                Kind::JarMavenCentralV1 => "7929ae847ea122c9b25f2066ac06a4425a46599b",
                Kind::RawSha256 => "711e1c2c88c8665fbea9ee8d02ae0653c0978b86843f37287b9dd2dcd35a1544",
                Kind::JarRawV1 => "1b47acd95ae2fe4e0eddd5592fc011f158b1e40f6e038ecb145e1786a5313e11",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-secure-sm-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "780ea6b495eb4740bb393108b03c134395bd5972989d8bba5f87f2801802e794",
                Kind::JarMavenCentralV1 => "d2d178b097862ed54dd71ac5a350de32532c0fbc",
                Kind::RawSha256 => "d6bcd1cc70c563babec902f2180187e00004a5c4ff023125b5cda166eeb68146",
                Kind::JarRawV1 => "2a87a0714c291b8750eb05edc5bf553f879c7a56a790749835771fa1521525b2",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/elasticsearch-x-content-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "f88d2b7a6743a0b0c53780e41d1acf8ed43cd132d357d972cbe265364ab4e641",
                Kind::JarClassV1 => "6cd4e8b535d4de6ec0206903260d275ddfe165d3d79c2542858d882ca742cf46",
                Kind::JarMavenCentralV1 => "71285d5949aa855d8d2d815e590d5926e242d6fb",
                Kind::JarRawV1 => "c5a8813548b43fec2ab5536e5eb6d175aa216136dff34aa5ff69bbf038fd9c89",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/hppc-0.8.1.jar",
            combined! {
                Kind::RawSha256 => "f540703478636d88f699f4666242e6fc9175a996c08ddceaf02106517b970406",
                Kind::JarRawV1 => "97acc4f0f2b9350ea1771ccedc9c22b9220730160e8e924d4b59339898d1d3a5",
                Kind::JarClassV1 => "ebb54d52e93451f08dfd84367e8702073c52048a5b124653ee2208cd8e3b263f",
                Kind::JarMavenCentralV1 => "ffc7ba8f289428b9508ab484b8001dea944ae603",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/jackson-core-2.14.2.jar",
            combined! {
                Kind::JarClassV1 => "5e8e302f66351bce9b1b41b71b79baf7f51d6dd16139a8e41866faf5cb0bab0d",
                Kind::JarMavenCentralV1 => "f804090e6399ce0cf78242db086017512dd71fcc",
                Kind::JarRawV1 => "ee2c5dce9f005f530afd9739d6217862e8bfe5b1c29373a06b4d3859a978a9be",
                Kind::RawSha256 => "b5d37a77c88277b97e3593c8740925216c06df8e4172bbde058528df04ad3e7a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/jackson-dataformat-cbor-2.14.2.jar",
            combined! {
                Kind::RawSha256 => "c942726863a8b7e0483d30d9213f9dadc8b07eadf0767003a4fe6dc56daa7135",
                Kind::JarRawV1 => "c8e623d8faca75b3da9cd349007ac6437e299d0b5fe61ca84af8a3e7cac238da",
                Kind::JarClassV1 => "bccc65c47461c575d838e09f858cb71c2c8e39c4327dc3509e97a05aa6295e7d",
                Kind::JarMavenCentralV1 => "5999f5365e27b63b27a52e671d15c0f8f8346873",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/jackson-dataformat-smile-2.14.2.jar",
            combined! {
                Kind::RawSha256 => "9c279bb29770de09289c14cf8862dd195112687cd7fde815919f54a9157ce213",
                Kind::JarRawV1 => "46029d27b2bab2ee8a070414624f9dc3a4166b84e0f7f6badfe7faa5e26f2cfa",
                Kind::JarClassV1 => "8a4217c57e9b03c0f6e90c02b84d0b947f02939fb4572d736dd8fcbef13ae2f7",
                Kind::JarMavenCentralV1 => "d03e3b991a1cb96357b98d07fcbe42d3d5c0b496",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/jackson-dataformat-yaml-2.14.2.jar",
            combined! {
                Kind::JarClassV1 => "b459a0a08a8c58a0180acc21787d11af4ec552021697f1d0c7a8bf211e9d2da4",
                Kind::JarMavenCentralV1 => "cc9a25c1f4212562dcb2fa33dd8ae179ba0e6a4e",
                Kind::RawSha256 => "5c3a0a71d0339529c80ae771497b20fdc0fa7cb67c772f99af5935927560006a",
                Kind::JarRawV1 => "3398200928912cbc179c0c4d831b8c43cc3773306e486707c1dd959096bfbf6a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/java-version-checker-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "1b661649431bf66a202b3f5d2bae0eb0fcb3f8fa087928351ca8a97f2aba4545",
                Kind::JarMavenCentralV1 => "943b734edcbaa6e4c3fc7719e82db8f020176d2d",
                Kind::RawSha256 => "ef0ca825e2f2bda117f954a795e75c287f5cef40d00c76060a355ea729e6c348",
                Kind::JarRawV1 => "6d222e8ae6ed9dd4dc549b2694f972729446f5d09501531d15a17fa279a3f355",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/jna-5.10.0.jar",
            combined! {
                Kind::RawSha256 => "e335c10679f743207d822c5f7948e930319835492575a9dba6b94f8a3b96fcc8",
                Kind::JarRawV1 => "4d9fa8b2674038b2216d11345637883005c3f3b36c6df2caf98cb4b4505531cd",
                Kind::JarClassV1 => "3182c4cad937014c678e251f59e2c5807ec98ddf45dbceda25fecf2e2f698d42",
                Kind::JarMavenCentralV1 => "7cf4c87dd802db50721db66947aa237d7ad09418",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/joda-time-2.10.10.jar",
            combined! {
                Kind::RawSha256 => "dd8e7c92185a678d1b7b933f31209b6203c8ffa91e9880475a1be0346b9617e3",
                Kind::JarRawV1 => "8cdcb8920fe090b6c05e6857c98fe76d64e77952277b8ea48b921e0cd20249a5",
                Kind::JarClassV1 => "2ad59c72014c59354a69407845e1af9f487a61988e1abc471f57191fab868885",
                Kind::JarMavenCentralV1 => "29e8126e31f41e5c12b9fe3a7eb02e704c47d70b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/jopt-simple-5.0.2.jar",
            combined! {
                Kind::RawSha256 => "457877c79e038f390557db5f8e92c4436fb4f4b3ba63f28bc228500fee080193",
                Kind::JarRawV1 => "cb9c567ee8cb2409fef0287fc4418ff3d7bf54d1c7179ecdd58e2702dca4118a",
                Kind::JarClassV1 => "4512e3d8881b866ff8302e154c2a53f25353c984f14499db71af35850d527275",
                Kind::JarMavenCentralV1 => "98cafc6081d5632b61be2c9e60650b64ddbc637c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/log4j-api-2.17.1.jar",
            combined! {
                Kind::JarClassV1 => "e456420a1a09c1e3ed22328c067c9d43d1ab969d06fa7529ed92e7df894764e6",
                Kind::JarMavenCentralV1 => "d771af8e336e372fb5399c99edabe0919aeaf5b2",
                Kind::JarRawV1 => "85afc83a396aadb819b3dc39886ede6356677811b4f74a3e1a1e77cf1d631792",
                Kind::RawSha256 => "b0d8a4c8ab4fb8b1888d0095822703b0e6d4793c419550203da9e69196161de4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-analyzers-common-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "d4c73f31b1267f22eccd55f117a0ad86e705bb5284b796fd9408aedb48591d29",
                Kind::JarClassV1 => "a3d58f1366ea2d755db2e0f0ce89fea089b363765076f806ae3f0892a530438a",
                Kind::JarMavenCentralV1 => "e50af506f271a3f7246da054a2569b42ff73abb2",
                Kind::RawSha256 => "1cdcc5a2d9cf4ffaf12fbf24bc2a18f2469cd295b60470ae8b97d1aa85dbad6f",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-backward-codecs-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "78bd321ca006136c777f94841703de765592a3daed70b42c14de215b28fb428f",
                Kind::RawSha256 => "38e72688eef81efffb9c5ea68918f4d1adb2eb0de64ce6a8222abee036eb63cf",
                Kind::JarClassV1 => "f5cb54cb9f9291c9719b92335ab46a082fb4ec33db736260cc7aeb33f710b4a0",
                Kind::JarMavenCentralV1 => "04fafe3e150805be67778ba5cdbae2560c893ad8",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-core-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "16e949a443e6c2ac86258f6754539bd518fca2ab2615147b82d8b3e6b093181c",
                Kind::JarClassV1 => "9ebcb4f23a1c7198faac110ece5d774891cf2fa173c936671830466ba2c434b2",
                Kind::JarMavenCentralV1 => "75dcf930ece95581af3f31af5692e4963fc7ad8e",
                Kind::RawSha256 => "78a61d0b843c1cf1fe5be380a4d3a4c1602d3fbba4ca1185da8797c9bb115483",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-grouping-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "f09a0215c8b5812bf0b003b8b9989216afb9af56e1869eb636cb65ec803b1fe5",
                Kind::RawSha256 => "c87fd0c311b8c712088536faf1dc2254a01a0d7c73e9ed565caa08ddc67e2500",
                Kind::JarClassV1 => "59e390cb42b732ffe97f5d15c79e799d28ae30972db0a3ab69e2d740fc7274d5",
                Kind::JarMavenCentralV1 => "538a2c45b1d99bbe14997996639b61fb6b919ab1",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-highlighter-8.11.1.jar",
            combined! {
                Kind::JarClassV1 => "d2f0c25a7c0c08b5e36bd76a0796fd61344bc240351b0bddd0dc7bfa6ab7988d",
                Kind::JarMavenCentralV1 => "558a389a0d68aa7cc4c26b9128c72d0d269d471f",
                Kind::RawSha256 => "c8e92e01b7443b2fd1698ac7b260b632197175143e13f0364b6f7258b9447307",
                Kind::JarRawV1 => "f09a910b20cfc4a42aeb62ba99875670357f8274ebd375a378a96f95ab86d5b3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-join-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "20a1912bfa2283519ee9c4b28eb559c0757187880e6783e04779ade44eb16757",
                Kind::JarRawV1 => "2d04ce13216b56953b7bd3326e4aa072697b6b1e10a5601cca903dbed56a187c",
                Kind::JarClassV1 => "672e51858da6844b76e1ae711751741fa0879527c872ae10fbdf83b496bd4267",
                Kind::JarMavenCentralV1 => "281e6c5432b7528f132be12f126b2986c3385906",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-memory-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "d12dd9296186e6b5a9c43ee95c7028102f543e0e9eaab9385419e759dc6611c8",
                Kind::JarClassV1 => "1edbe9bc4dc89baefa6a173fd5183475ae6c521e448f1aa2c707ff258f4f1ac1",
                Kind::JarMavenCentralV1 => "e6e3a3e19bf9c7860c31966d52dfbf16909f4cc0",
                Kind::RawSha256 => "b948478fe2e8e7f94fa7f533a14a526720701fe98627214c93d924faa4be78de",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-misc-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "e7d891cd5f488afbd5b5b81ce42cea062ba1e832f55c89441797e1bcfb2a695f",
                Kind::JarClassV1 => "53e462f256988941598804f5df5b280ccf0698a4a0167107089feb02ba627813",
                Kind::JarMavenCentralV1 => "c412eafb48b9623d1046d72d7a2139d69c80548b",
                Kind::RawSha256 => "4a1102663cba35676616038b516d487ee3db9802fade99649e14f09c8b412cf3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-queries-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "11fb2e90da5b4e6a6c26120bb80a2937a20a585d32236ed7c277048ba65f07ca",
                Kind::JarRawV1 => "a74dd3980e3a94e4194ad9da79cbb91b554e383f53acc2a0885cc62186e0a844",
                Kind::JarClassV1 => "26f69dc1ae4fc85526033d5064776bab323c5c1873f2658de302281b838414aa",
                Kind::JarMavenCentralV1 => "02a0abca16b31453cbd832ba27c96fce1208db5e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-queryparser-8.11.1.jar",
            combined! {
                Kind::JarClassV1 => "d77ae0518068d04fd76c85e01ddce9528bbd55ef233272e9073749eb969a7fb6",
                Kind::JarMavenCentralV1 => "5a44df2cb26fa1e0e64be53fe474c7f1d5a3e634",
                Kind::RawSha256 => "23abf022a19e609fe3ca421ab6b6868a3250974d31c5b92f9879d97c127a77b8",
                Kind::JarRawV1 => "f79f96d3a9ccf7fc9c438178f1600c1ebfe09fe29dae04068a9ad2fb83db76c0",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-sandbox-8.11.1.jar",
            combined! {
                Kind::JarClassV1 => "9d88cfebbc1e31cf6e9df3b5355c6fca570e9883a66d259ae06ca015d8cc8763",
                Kind::JarMavenCentralV1 => "bd4392f44a5f7ed798baca88896c0ff6a428ab90",
                Kind::RawSha256 => "28bee2711947cf3a9957f3f77132ce37457894c1fb468b0a20e9a95788b11c87",
                Kind::JarRawV1 => "849df1f98ae16c6ebf0379c67dfe326a2ceee42bbc7db0da4ae3c9856d6c4530",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-spatial3d-8.11.1.jar",
            combined! {
                Kind::RawSha256 => "fb98a09f8da5e912b2c5dcfe6353373a3309cdbf11e64031462cd9d19bd89256",
                Kind::JarRawV1 => "c907dbca85ff43726c831dcd6773486dd6f6b29f5d53f0babe9c193053a2c7cd",
                Kind::JarClassV1 => "ef347f6f1e3107992a035ce434d9db09b7a60ec130d7035bc00d0c783e1c313f",
                Kind::JarMavenCentralV1 => "29b1d2edfab05beb0014e6582d53c74129c5bc03",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lucene-suggest-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "b678df5cf98757e9ed6af8143f5b7b2448d94f907f54d48520817b899092a787",
                Kind::JarClassV1 => "b75b3194e53db4a56b7b847a006eb08631b1ca6e7bb8019ba1836da5d511d17b",
                Kind::JarMavenCentralV1 => "60bbaf1907cd3ec583df6cd0a86d62c6b25222f8",
                Kind::RawSha256 => "e328709455e56a1b41c270e69d77ee98c116d27fc5dca6cd3972c45dadb3d23a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/lz4-java-1.8.0.jar",
            combined! {
                Kind::JarRawV1 => "c5d86569d7e758f1cca5d639964845b7d5a0550e6165a184145aca3bbbb5f66a",
                Kind::RawSha256 => "d74a3334fb35195009b338a951f918203d6bbca3d1d359033dc33edd1cadc9ef",
                Kind::JarClassV1 => "e5d613383bc81df7584c0cecc045daeba675d80c17874ebafe1760fdd5bb37f2",
                Kind::JarMavenCentralV1 => "4b986a99445e49ea5fbf5d149c4b63f6ed6c6780",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/snakeyaml-1.33.jar",
            combined! {
                Kind::RawSha256 => "11ff459788f0a2d781f56a4a86d7e69202cebacd0273d5269c4ae9f02f3fd8f0",
                Kind::JarRawV1 => "dcee9e292c05e428c43951d237db7ab32089b4c6d4c51a90e8eb226867fe30d6",
                Kind::JarClassV1 => "a0a7b22ba470b11ab71ae8bd0be0ef8816355a9a9e8fb2a427991dfd513ee360",
                Kind::JarMavenCentralV1 => "2cd0a87ff7df953f810c344bdf2fe3340b954c69",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/t-digest-3.2.jar",
            combined! {
                Kind::JarClassV1 => "db500040a066ab5003b524b31dc0afe4c78894ec91be799fe815c11cb6ee1dda",
                Kind::JarMavenCentralV1 => "2ab94758b0276a8a26102adf8d528cf6d0567b9a",
                Kind::RawSha256 => "03db291a8887b474f90db67bfb1f92d084e990150037e231babbb374ee11d7c3",
                Kind::JarRawV1 => "29ceca9b46833a1baa93b205476f63f196d53b2349e655c8df6cab2df455b4fe",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/geoip-cli/elasticsearch-geoip-cli-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "1cab1a1dac91b895de3bd802c44ffa68fa0762d1eec0c4767f7d11b9462d99e6",
                Kind::JarClassV1 => "bee4dc9a79c385e4148a536e05bb3658725dbcc9308c8574725ba8b0c612ce14",
                Kind::JarMavenCentralV1 => "640800fb3c016bab14caedec22ae5cc58451afae",
                Kind::RawSha256 => "6e2ffd325a846b42859d8875d270899fde3aeaacb62b0edb001290284f7534ca",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/keystore-cli/keystore-cli-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "b01f752b5fa626e509805aef7e1e5a7ccf9b86a1769d3398b9b2755d6de694dc",
                Kind::JarMavenCentralV1 => "1647736235fd754f41cf90749a60bf754193d85f",
                Kind::RawSha256 => "5386f76c7b47f2d972b1733e48a510457b661736184fb01db38ffe8c11216071",
                Kind::JarRawV1 => "36994a58dde2f18f17038369043b8424331401f6099fd0d0740c635bd11e5345",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/plugin-cli/bc-fips-1.0.2.4.jar",
            combined! {
                Kind::JarClassV1 => "8d326651df98de1c4d14e388f1078fab914731b32818dab194d643427956a580",
                Kind::JarMavenCentralV1 => "9008d04fc13da6455e6a792935b93b629757335d",
                Kind::RawSha256 => "703ecd8a3a619800269bc8cd442f2ebf469bd2fe70478364f58ddc6460c35f9f",
                Kind::JarRawV1 => "b5f49ba8cd4515c074b509752a07bc612a22aabe3597cef7ca5b25e3017ba835",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/plugin-cli/bcpg-fips-1.0.7.1.jar",
            combined! {
                Kind::RawSha256 => "fea1a096c098395eb67d48700c349d5f75321ef0c7c6af9198bc38f4cc836622",
                Kind::JarRawV1 => "84dc8d0f364d88bcee819556cec05e061d3957c691e4d8d6d4272b63a0e9c601",
                Kind::JarClassV1 => "761114c2a1879f99c7ab1223b7844a8546a9aa9affda7f31d8ef6bf6ab1b0eae",
                Kind::JarMavenCentralV1 => "5e1952428655ea822066f86df2e3ecda8fa0ba2b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/plugin-cli/elasticsearch-plugin-cli-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "72f7efdfbc379afdfc26954434b6c94e0931dc24174a39b83a119a95aa6a90f4",
                Kind::JarClassV1 => "765ab61850cb53f0fc9972f7e3f86581781ea1d45f8c8aff8725121f0ca7326c",
                Kind::JarMavenCentralV1 => "02b9aaa832a7192c49930b137f4b4cf5787830ea",
                Kind::JarRawV1 => "6bed8388bf0b32fd42e3f22a8d30af9c0d8dbcf9e7694e8958b7855c1d16e4ad",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/security-cli/bcpkix-jdk18on-1.77.jar",
            combined! {
                Kind::RawSha256 => "1ac7fe8efd5b2f38cdc165be5a0675734fe44808dab92707201f03a535d6f1b8",
                Kind::JarClassV1 => "f86e6babfe5da322d71ff19dd18cab59be354a1189c7a965e4310c0744bc0b60",
                Kind::JarMavenCentralV1 => "ed953791ba0229747dd0fd9911e3d76a462acfd3",
                Kind::JarRawV1 => "ef920856eced55236d35aa9c8088f7c0f6e0a069c25079f9e1eae55b01a45d8e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/security-cli/bcprov-jdk18on-1.77.jar",
            combined! {
                Kind::RawSha256 => "dabb98c24d72c9b9f585633d1df9c5cd58d9ad373d0cd681367e6a603a495d58",
                Kind::JarRawV1 => "ab87332836409c193e5ce5661a88509186210407572643d90db9ddb6250fa7dd",
                Kind::JarClassV1 => "24e71ea245e48703297ff9910cdc7dd068d03e7e9f6e99571f437b6e8fd74324",
                Kind::JarMavenCentralV1 => "2cc971b6c20949c1ff98d1a4bc741ee848a09523",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/security-cli/bcutil-jdk18on-1.77.jar",
            combined! {
                Kind::JarRawV1 => "e21cc80935d4be2352ac03cbb3ff1ef3cb6c19475c45900474fa042288eb4cc0",
                Kind::RawSha256 => "947673bcbc5a8dde2d2fa688a5b7598d0ca6e2a74a7ea30cd93f04f6b3ad68f8",
                Kind::JarClassV1 => "f618a68b0a51368b25f6af06e2818d8cb55854c3428546b0c28d3c00dba02cdb",
                Kind::JarMavenCentralV1 => "de3eaef351545fe8562cf29ddff4a403a45b49b7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/lib/tools/security-cli/elasticsearch-security-cli-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "7670dc9e05baba8828e8d005da86fbbad1e1be81eeb53dee8b1e17ffa589ec57",
                Kind::JarMavenCentralV1 => "2e865a6d80b4f540d4960eadac4218a1b07f9caa",
                Kind::JarRawV1 => "b8b7fa6699e519c9473876012a0ef5590aa9211e0f59a4c184060c70a3b37dd5",
                Kind::RawSha256 => "1135c281bdeb2bd20771f98c1bb992cf49b6884093f8a713571532248aab28d3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/aggs-matrix-stats/aggs-matrix-stats-client-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "aa2a5bcac58731f1c102036b5030ba2b520728591696dde4dc45c6623127fdea",
                Kind::JarMavenCentralV1 => "a95e2b2438857705492134bf0392cbfb1af748f1",
                Kind::RawSha256 => "9bfe710b4762e6bd3fb28dcc07f9ff2aa4f36fdb2aad1dfc5bd6505ad9b4b404",
                Kind::JarRawV1 => "be8acdb650fdc844979f14fdd4ec70da12f67e023fa174bd0c6fa10ce3ec294e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/analysis-common/analysis-common-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "7e079430c48683162a073207cdbbb2a1c9ab4d1847804ee7d7f09ec0ede6c37b",
                Kind::JarMavenCentralV1 => "cf03bc26590825d1023eb422db9b7c85aee6b046",
                Kind::RawSha256 => "7a5c9fbb5ccbbfc20142877ea0054e1c4a6cce44a1c8b271d8404dd8d866bee8",
                Kind::JarRawV1 => "3fb9b8575445b7ae64c8ac1abb9cbd4e993160e0593d80ab42ba6efe19fa7fbb",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/constant-keyword/constant-keyword-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "b6128cb401331f458f6a74efe1c8ac60dcaed1c2ff2088c7819a228af5801f6b",
                Kind::JarRawV1 => "e158622b56e5c3eb103087ca4ef09df33608669839fa56ba7db42590f0199beb",
                Kind::JarClassV1 => "217598f42ef9a99a0973ee0c2d9d5b42bd620a31619bb3b47d3b9121fb4a2911",
                Kind::JarMavenCentralV1 => "4ed1042f4703fc2bed88fc59260038ff9cc5ff18",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/frozen-indices/frozen-indices-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "08a4c2ccec8e872873f0e20fc497700d1e7342e628deb04f11d26762b11f6d38",
                Kind::JarMavenCentralV1 => "7f2ae6f7b7a5d8b69bf6d3aca5b1b89065687970",
                Kind::RawSha256 => "423a2cb10321ea3e9fc9e76c1310cc96d742cc49071c4ff19bfdb3458b883ebd",
                Kind::JarRawV1 => "399687e9927970ead0f12820aa55a7ffc15ccff89cec61a3b966a46ae46c1be3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-common/elasticsearch-dissect-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "63c18fa87875fab72bd469449040bb9884204414557e5f6ffdcb8da9cfa98e87",
                Kind::JarRawV1 => "4b0570e1077894effaac3b5efde9bc9b03e15e0ef88d9a58df99f1ba476020cd",
                Kind::JarClassV1 => "95041177b60ad6b80438591597dd00a08188314bfd02b35a7ece3f2ec9b37534",
                Kind::JarMavenCentralV1 => "11d3441c1516d8960fed65539b985f4ad274b572",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-common/elasticsearch-grok-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "e6b6ef74174dec6b1c50d3cb6bf82d73413e56d9d038ddc7e06a26e1eb6fea14",
                Kind::JarClassV1 => "fee1097201de7908879f65cdaf0fc54bc60a4e023450ca1f9eaf3ffb7b14bc84",
                Kind::JarMavenCentralV1 => "69afea1244752b8cd4b212e3c942e4a1ec3191c6",
                Kind::RawSha256 => "a2267aef94e0a815521d5cb233aacb512719cd96721f64f056dbc085397f761e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-common/httpclient-4.5.14.jar",
            combined! {
                Kind::RawSha256 => "c8bc7e1c51a6d4ce72f40d2ebbabf1c4b68bfe76e732104b04381b493478e9d6",
                Kind::JarClassV1 => "a539ce4f6e2cb59c8b36372e5951db869d2eaf8f45f04eae72255fb9f38c6f16",
                Kind::JarMavenCentralV1 => "1194890e6f56ec29177673f2f12d0b8e627dec98",
                Kind::JarRawV1 => "ce7270d53369efb990e03e5075b51f9c732fcb088f9cb55eda788cdf46aa69a6",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-common/httpcore-4.4.12.jar",
            combined! {
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::JarMavenCentralV1 => "21ebaf6d532bc350ba95bd81938fa5f0e511c132",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-common/ingest-common-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "13be878c5a9b69255e27859fe6338a9c0da8f9b5b08c1d53738be45e83e95889",
                Kind::JarClassV1 => "910eef238d61a20ba190023fc5d3c91361640df3a042dd94b08a3eef524612ed",
                Kind::JarMavenCentralV1 => "4fc9a8c5bab3e9d1579e3e7e20f63c02ae12ba81",
                Kind::JarRawV1 => "170b3576caaedfc774e30501a50956804f71d62bc6068fb67e41993bf7de9e65",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-common/jcodings-1.0.44.jar",
            combined! {
                Kind::JarClassV1 => "4dd715e0832222e95234af07d22995a0cc40fdc7328ec88b355877d060650532",
                Kind::JarMavenCentralV1 => "a6884b2fd8fd9a56874db05afaa22435043a2e3e",
                Kind::JarRawV1 => "3ff823499e3d4fd4815efbf4a54311290f5a64592d944e901f8a2940009b35dd",
                Kind::RawSha256 => "49190d6ad09056de57d7ed41ed5b4b105e033557b5dd170702decdcf05ee341a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-common/joni-2.1.29.jar",
            combined! {
                Kind::RawSha256 => "aa4b71415682f3d7fa44083cd94a9ec48478ec3b9c30947b4152913d41b1004d",
                Kind::JarClassV1 => "e9629b51e39e64cd10c5a7a7e10dbb0c5428ce4006a864da5fba4e1a1ec03ff2",
                Kind::JarMavenCentralV1 => "3cb751702e1194ff24259155db4d37e9383d4320",
                Kind::JarRawV1 => "5f5c1894e9780c6e408de3baeee7585886d8fbc3ae73b6af25722912567cdcff",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-geoip/geoip2-2.13.1.jar",
            combined! {
                Kind::JarClassV1 => "ca8671b9d93804c1c4a51d52661c75b0ced57bb4d0b00568007af6bfb7c2d791",
                Kind::JarMavenCentralV1 => "f27d1a49d5a29dd4a7ac5006ce2eb16b8b9bb888",
                Kind::RawSha256 => "cd0ef447c0e0466d38bc75c1230b7c79b7aa8d353aa25e14f552ef8f3fd0ff3b",
                Kind::JarRawV1 => "e1db7342c4d478d1d857a8c4a875c2454c6fd9070b1e0a0eea0fe963d294649f",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-geoip/ingest-geoip-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "80a566fa3297dde8bc4069314ef61f7c80e53cd7ba3efb9b366828a2a03a1b44",
                Kind::RawSha256 => "86a4efb4d34800ec733115043cd580e57208f5acc05088d28d94433b207c8dd0",
                Kind::JarClassV1 => "fffe1d0b7f3c0f97d17c8bfd5ecc0f0f0a46738ce8bfbc847b3af4fd0572eca9",
                Kind::JarMavenCentralV1 => "5e170582b5dc27ec35fc5750c4389aa89724f8e7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-geoip/jackson-annotations-2.14.2.jar",
            combined! {
                Kind::JarClassV1 => "b2af55aad6e191a799549d6cfdf506c9a0c23ed50ff9451e4884ccc7cf37a82d",
                Kind::JarMavenCentralV1 => "a7aae9525864930723e3453ab799521fdfd9d873",
                Kind::RawSha256 => "2c6869d505cf60dc066734b7d50339f975bd3adc635e26a78abb71acb4473c0d",
                Kind::JarRawV1 => "74351b16e57099e9842722b204527ff07e3fc2388f527be5f0c463d6dc7aedbc",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-geoip/jackson-databind-2.14.2.jar",
            combined! {
                Kind::JarRawV1 => "4be78541b9f3ff7c2558b915caf467a6f248e6f6ef3bc36c46b17a9ac3acf380",
                Kind::JarClassV1 => "2ecc4851818fe11ea80914ceff6fba24da7b071f90de8a750b741a1f08c85a8e",
                Kind::JarMavenCentralV1 => "01e71fddbc80bb86f71a6345ac1e8ab8a00e7134",
                Kind::RawSha256 => "501d3abce4d18dcc381058ec593c5b94477906bba6efbac14dae40a642f77424",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-geoip/maxmind-db-1.3.1.jar",
            combined! {
                Kind::RawSha256 => "cc955e897306baadad7b16d58918bb020c847010d458a603a0e53b5a47bd0961",
                Kind::JarRawV1 => "4edd6528973323c7ca9a679cde8faf0b80b48d04810eefceeb859b69e427e427",
                Kind::JarClassV1 => "02bd49521a6fefcf8ad678b4045934933e891a2904e7c00d435c82ca8270850a",
                Kind::JarMavenCentralV1 => "211bca628225bc0f719051b16deb03a747d7a14f",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/ingest-user-agent/ingest-user-agent-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "22cec5fac98e1a84d7c81bfde61233066e9c11a417853a7d4b8350ad1cbd14fb",
                Kind::JarClassV1 => "38447602f227c0912e968e1a43b17c618d2242c477fe2714e4508c70b5feb14e",
                Kind::JarMavenCentralV1 => "8c7e1567e69738e2f2af4135b1adf85d5a7e9fe8",
                Kind::RawSha256 => "6f8f59b050840c57bad8b34ef1ca368f5e5dc50b8bd4be49b47eb42aa80035d7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/commons-codec-1.15.jar",
            combined! {
                Kind::JarClassV1 => "130c47d01c72e33a0332a95d4a1d8718a2465ac015ec6b3863174618caf58e2f",
                Kind::JarMavenCentralV1 => "49d94806b6e3dc933dacbd8acb0fdbab8ebd1e5d",
                Kind::JarRawV1 => "54324aabfbe16b3cf75c314676de4524a6d2cf3250ac8ea9a61507fc8ff79bd3",
                Kind::RawSha256 => "b3e9f6d63a790109bf0d056611fbed1cf69055826defeb9894a71369d246ed63",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/commons-logging-1.1.3.jar",
            combined! {
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
                Kind::JarMavenCentralV1 => "f6f66e966c70a83ffbdb6f17a0919eaf7c8aca7f",
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/elasticsearch-rest-client-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "08b04d8bc63a2cc976bdad6b6b043d3f17e3c0437450ccea6be2c7316d806c8d",
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
                Kind::JarMavenCentralV1 => "810c3578e58f02e112a918f589b153222891d41b",
                Kind::RawSha256 => "91cfab57383a284c826a6dd513dcfba309aea0b473af18582552c18c8b5dd078",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/elasticsearch-ssl-config-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "b2b9e1eeb2c446a19c4282d941aba1c26f453959bde71c33f779546923182c22",
                Kind::JarClassV1 => "bfa8378ae967a7d0e3dc1ffc52b65da2eb6afe95763c54dbc3e7c97f9169c5f6",
                Kind::JarMavenCentralV1 => "27e77cd3bb6f7b8a6651a157ec3f18147a476d33",
                Kind::JarRawV1 => "ffa840a844f490041aa65bbc4763e3b74b926bcefb8f97eb19f665ee9b12b1d2",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/httpasyncclient-4.1.4.jar",
            combined! {
                Kind::RawSha256 => "50e981a8e567a16ebdad104605b156540a863459fa127b8ba647f310dfc83ef8",
                Kind::JarRawV1 => "45f280f8ed6c9600a41afc2bf6f73dbd96bab37c0cd0bf66e52f916c9c57a9a4",
                Kind::JarClassV1 => "77f6448014b48e685332f32f0aec9c34383bb7a69ea8e2586f2a738418be6669",
                Kind::JarMavenCentralV1 => "f3a3240681faae3fa46b573a4c7e50cec9db0d86",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/httpclient-4.5.14.jar",
            combined! {
                Kind::RawSha256 => "c8bc7e1c51a6d4ce72f40d2ebbabf1c4b68bfe76e732104b04381b493478e9d6",
                Kind::JarRawV1 => "ce7270d53369efb990e03e5075b51f9c732fcb088f9cb55eda788cdf46aa69a6",
                Kind::JarClassV1 => "a539ce4f6e2cb59c8b36372e5951db869d2eaf8f45f04eae72255fb9f38c6f16",
                Kind::JarMavenCentralV1 => "1194890e6f56ec29177673f2f12d0b8e627dec98",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/httpcore-4.4.12.jar",
            combined! {
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::JarMavenCentralV1 => "21ebaf6d532bc350ba95bd81938fa5f0e511c132",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/httpcore-nio-4.4.12.jar",
            combined! {
                Kind::JarRawV1 => "24e4f97e5ff3f463dc40dbdd72e84f11c506e479c31e5a966dc92f68367ef631",
                Kind::JarClassV1 => "46e99f783b2e361033e0b5d74193643715b4faa2037d6aaf39aa1bf93fc05adf",
                Kind::JarMavenCentralV1 => "84cd29eca842f31db02987cfedea245af020198b",
                Kind::RawSha256 => "11448f4b5c7f13d9396a67b33aa938d05f660665e0f14fd08e25acfd3c20ae80",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/kibana-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "77eadfe84ff6075af9b6003ccdea8437872a617dbe1e69b932cf78042975e073",
                Kind::JarClassV1 => "42a53d242bbbe1adf01250b8c0ae92342dff04b4688471e3d4a4ef404566b618",
                Kind::JarMavenCentralV1 => "d52b645d9513f2d5e95b09c842c94f33aea9ccbb",
                Kind::RawSha256 => "c66bc78de4ef55598ee420735a36156888e0bfa0bd696a356d81475133b25b61",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/kibana/reindex-client-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "711a6e126ab14e65b90a7bce7adb29004fbb365afe7563aa2160bd47c43cb2ef",
                Kind::JarMavenCentralV1 => "a614744ddcfb609b4601e9c42cb226b2f0f9cd44",
                Kind::RawSha256 => "16cad00497cfd6774128943c04a54c160ba6a3f6d7d7711256e1c1cada75a926",
                Kind::JarRawV1 => "5179d66d44a1049c11043ac0063c3074df838479dde9345dc45e2235b58f265f",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-expression/antlr4-runtime-4.5.1-1.jar",
            combined! {
                Kind::RawSha256 => "ffca72bc2a25bb2b0c80a58cee60530a78be17da739bb6c91a8c2e3584ca099e",
                Kind::JarRawV1 => "74f6c74696df093e7004416f9073ef292b6ecb8f938553f914721b70802b7ec6",
                Kind::JarClassV1 => "7e61d6987054a4d1dbf1616459e780cb57162a62d564f441ca3b5bfd323a1550",
                Kind::JarMavenCentralV1 => "66144204f9d6d7d3f3f775622c2dd7e9bd511d97",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-expression/asm-5.0.4.jar",
            combined! {
                Kind::JarRawV1 => "6c75e4cc2dd9e51523115291b81a99b91fa95693a4768aaa22c76007fea80b1d",
                Kind::JarClassV1 => "26ba53d2e2e49f7f223d46afff57586676b93a934cb1157ddeb244fb1a3b6f96",
                Kind::JarMavenCentralV1 => "0da08b8cce7bbf903602a25a3a163ae252435795",
                Kind::RawSha256 => "896618ed8ae62702521a78bc7be42b7c491a08e6920a15f89a3ecdec31e9a220",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-expression/asm-commons-5.0.4.jar",
            combined! {
                Kind::JarRawV1 => "74c6818b325f573cad4e4fadf0be091b6a19ddd17cad95f077cd5392a4b23ed8",
                Kind::JarClassV1 => "b8d0aabaf89d15aa5a6c32fce16a37f10615d5b753ce7d97449051cd41a9108b",
                Kind::JarMavenCentralV1 => "5a556786086c23cd689a0328f8519db93821c04c",
                Kind::RawSha256 => "532f0ea290b28651b18f03e375f1b5693e87bbf696a25879ea1f1182557486af",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-expression/asm-tree-5.0.4.jar",
            combined! {
                Kind::RawSha256 => "c3670fa836fac4cde743840f671a7d52a32eb0a844e2824eaedaf3a47f5c8399",
                Kind::JarRawV1 => "d056a33c47afa974bae98a863d8619ec19f84b299c2edb19b725f9224998acb5",
                Kind::JarClassV1 => "70128ccdd0720210ab2ef935cfb59cfaaee509403e3bed572c792a1d615ac06f",
                Kind::JarMavenCentralV1 => "396ce0c07ba2b481f25a70195c7c94922f0d1b0b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-expression/lang-expression-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "608058104cdac85daaf3871b51931d77ff48f401a8d715b58d11cc6ce841d7b9",
                Kind::RawSha256 => "e9134922e43334666de9026a2e390a02344ae1c4a2a30ca6d8935b1e07bc1016",
                Kind::JarClassV1 => "276338e47f30a2276fb108f5a62e4d29ba6aed481c2d4557922e2faf2af340e2",
                Kind::JarMavenCentralV1 => "b6c9bcf8f8afa6b1dfd4443f6b3ac84a72296346",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-expression/lucene-expressions-8.11.1.jar",
            combined! {
                Kind::JarRawV1 => "202d97b9c0a70e6c80e31772490c264d33060623d36757d5452b30c7f1bb93d9",
                Kind::JarClassV1 => "71173938dd67919a58a272abf6bdd31a0988d3a7e5d40262373b0f116f6123b2",
                Kind::JarMavenCentralV1 => "55d929f0bea58211fcc47085bdd04d04d8f29179",
                Kind::RawSha256 => "7b3ac33889a54487794ad7b7afe8c71b77280c37cf148a0cde082ffad2824414",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-mustache/compiler-0.9.6.jar",
            combined! {
                Kind::JarClassV1 => "636d900e125852c79678a168f1b1a285af804a3538bf94907437cd25e5df5e08",
                Kind::JarMavenCentralV1 => "1b8707299c34406ed0ba40bbf8513352ac4765c9",
                Kind::RawSha256 => "c4d697fd3619cb616cc5e22e9530c8a4fd4a8e9a76953c0655ee627cb2d22318",
                Kind::JarRawV1 => "07e52d443b6f1819ac0e9ca901d1b6a9232a9220d394d6979542864ad308eee4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-mustache/lang-mustache-client-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "b22bf08282db73ebe65bedfd8fad1a91edd6f812422a1a2769964345ef1d1ed7",
                Kind::JarClassV1 => "6d5c77f4dead86af7438a9f1365d93ae4a761ad861bfd8e80d4b132eec58a680",
                Kind::JarMavenCentralV1 => "073678ebfa96477aa72952763995026a6fbc733d",
                Kind::JarRawV1 => "ab0f11c8c65223c2213a71721732f4a1dd0f3c819162fc4569541ed0097435fe",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-painless/antlr4-runtime-4.5.3.jar",
            combined! {
                Kind::JarRawV1 => "48d8ab8b98757a6437ada2fa196645abc4be5cb192a3a9f50aa670875c5417a3",
                Kind::RawSha256 => "93bca08ec995caeaaf60bdf80035a0be8507fcdabd3c2618fd8c5aab4444a752",
                Kind::JarClassV1 => "8cdfae0e14be3bcd345fbbf3dfe2bee73e5e82ed21c259af47d8cbe13a1dcd29",
                Kind::JarMavenCentralV1 => "2609e36f18f7e8d593cc1cddfb2ac776dc96b8e0",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-painless/asm-7.2.jar",
            combined! {
                Kind::JarClassV1 => "f4c640b0e933f55e567e0c6ed31609515a1da985706e87ae597d8914a541c490",
                Kind::JarMavenCentralV1 => "fa637eb67eb7628c915d73762b681ae7ff0b9731",
                Kind::RawSha256 => "7e6cc9e92eb94d04e39356c6d8144ca058cda961c344a7f62166a405f3206672",
                Kind::JarRawV1 => "2bed47e7fee145b8cf2945dd07fadc2ed02b7e16c6c234c9bf4ee88ea4a6902e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-painless/asm-analysis-7.2.jar",
            combined! {
                Kind::RawSha256 => "be922aae60ff1ff1768e8e6544a38a7f92bd0a6d6b0b9791f94955d1bd453de2",
                Kind::JarRawV1 => "1e219938f4d0f103ea2e9f63dac1dcadb42dd7c16829a9acc6c7e98b1e57aef7",
                Kind::JarClassV1 => "2e62a2125a33e5a8252b34adcd9424f2a1c79ebd97f9731e700bbfc6e6c7af6f",
                Kind::JarMavenCentralV1 => "b6e6abe057f23630113f4167c34bda7086691258",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-painless/asm-commons-7.2.jar",
            combined! {
                Kind::RawSha256 => "0e86b8b179c5fb223d1a880a0ff4960b6978223984b94e62e71135f2d8ea3558",
                Kind::JarRawV1 => "55c8209eee3fd2df3c56572fde4c05870c4866c5d7220d7bb2fff5751d57b758",
                Kind::JarClassV1 => "4708669806879cc354718783e58a85ccbb4c28f566e680e1f7b6af65b052089e",
                Kind::JarMavenCentralV1 => "ca2954e8d92a05bacc28ff465b25c70e0f512497",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-painless/asm-tree-7.2.jar",
            combined! {
                Kind::RawSha256 => "c063f5a67fa03cdc9bd79fd1c2ea6816cc4a19473ecdfbd9e9153b408c6f2656",
                Kind::JarRawV1 => "1ee5d5e55da2307c6bbf3a5d1f84470c460c9b07855e05fd9d656116418e6062",
                Kind::JarClassV1 => "255dfc62fe4b3073327fb54353d7fee0a1636f3c87907cb43d1500774ab764e3",
                Kind::JarMavenCentralV1 => "3a23cc36edaf8fc5a89cb100182758ccb5991487",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-painless/asm-util-7.2.jar",
            combined! {
                Kind::JarRawV1 => "ab429a0e53bb8b36642e3c4739ef4624a31bc852032a64d3045b52c6a1f6e155",
                Kind::JarClassV1 => "4e78cf683c8701f0b16b188e577049a88545d2b8dc38175b59bc583a89f74606",
                Kind::JarMavenCentralV1 => "a3ae34e57fa8a4040e28247291d0cc3d6b8c7bcf",
                Kind::RawSha256 => "6e24913b021ffacfe8e7e053d6e0ccc731941148cfa078d4f1ed3d96904530f8",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-painless/lang-painless-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "796b362979443cf399fc2072a8191e31fe6e2443a51837a362ef3487a70f9ca8",
                Kind::JarClassV1 => "bbcac1125b67bebe6bee7d91d62fb1b72c84a2939ce3041b7da8543c8e4d7087",
                Kind::JarMavenCentralV1 => "30f811ed4fb7330d38a01aca968b986afc1e4264",
                Kind::RawSha256 => "85b08d4e6864536270afad69552e14566ebf46a0322bf8521fc9175aa126df86",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/lang-painless/spi/elasticsearch-scripting-painless-spi-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "3be5238d3a1de31b39e20f6876bd6166226059fab979c84a4bbbc5625b70939a",
                Kind::JarRawV1 => "49bf227cc847bb290906be97fbbcf8e075a4924e01eedac95048a7c4e88e7dae",
                Kind::JarClassV1 => "bd8af6c013f5e874f59969799d3ace10b86fd8536db616fd6d043a6c979329d4",
                Kind::JarMavenCentralV1 => "7d522a47ec5ec63aa0f45087e7ef7f1696fe8e7b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/legacy-geo/jts-core-1.15.0.jar",
            combined! {
                Kind::JarRawV1 => "4462ea26e83f4af4dcd63f3bf039478a1982bccef562f637865439fb5aa985df",
                Kind::JarClassV1 => "dcfec9f02a26f99933123a3e656e61783e96b304f0c2e22060d98f158832777d",
                Kind::JarMavenCentralV1 => "705981b7e25d05a76a3654e597dab6ba423eb79e",
                Kind::RawSha256 => "00102cde26c457b81fbb0248e4f8845884243caba0dc9b7fb42e0ea877383bc1",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/legacy-geo/legacy-geo-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "7afd0a91ed89927904c7bd1d064327d069a5781bf9a60ebe3258390f1f53a289",
                Kind::JarClassV1 => "3c8db452302434296a9f650a994e96efd392a19088a0fb24bfa9cf02a7fedbe6",
                Kind::JarMavenCentralV1 => "1d807b8e39e8f4a24a74c77fcd4875719fe134b3",
                Kind::RawSha256 => "e628c25f0aa52e7dedc7afa2ffc9f931797e8aa71e9c9f8ebba87fc254235db7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/legacy-geo/lucene-spatial-extras-8.11.1.jar",
            combined! {
                Kind::JarClassV1 => "7d0176090e6d72b340cebbfd307bcee0fb9b6c6cbdbe8e45276fcb76b0f67a4c",
                Kind::JarMavenCentralV1 => "9845aa3aa24196aa8d148d8c1bb5384d827eaf51",
                Kind::RawSha256 => "ae82f78c01fe969930ecea104a9dc78bc210fc45b55c3d695b66b80468cb69e3",
                Kind::JarRawV1 => "68c7e499c30111de061cbdc87998c48f92d1a8377774723b30ae069326b27a67",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/legacy-geo/spatial4j-0.7.jar",
            combined! {
                Kind::JarClassV1 => "d46c77130b99e0fe578ac7ae87e44bff6e3d42e16193bb02177064c4c1a92ef9",
                Kind::JarMavenCentralV1 => "faa8ba85d503da4ab872d17ba8c00da0098ab2f2",
                Kind::JarRawV1 => "998f9109b0c4d61ef344693eedbffd672cf0a0602e271e18ed9a8101022fd1a3",
                Kind::RawSha256 => "9adccb1d87f7e0be70567b952c65552607e2dcbde32a1579a8a639bdcfa1a3c8",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/mapper-extras/mapper-extras-client-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "001fa1d7f8c7ef8d4ea9c2f98b85ca021b0f3b339c006e252a89d05b9c9bc6c3",
                Kind::JarClassV1 => "f62a000140ab8c7c0c9292c74c52dd7e39f3b22a2e2e7a616482c44a4122003a",
                Kind::JarMavenCentralV1 => "11f0b69717c3f2f74180718979902386b8c828b8",
                Kind::JarRawV1 => "12268c7514be1129cc21c2303c12f1598756cea4b83f6fc09eead1844c76fff2",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/mapper-version/mapper-version-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "86555ae5b34c6c1553e9bf2eaf06657f26d58ba64dd24b11ef1eb3d5cad6dc27",
                Kind::JarMavenCentralV1 => "3004e6d3860777cdcc41362cf692eb2ad4e6e449",
                Kind::JarRawV1 => "80eb1b35fe9ebd3158f44c12c7f5a68f0ca76c3e4d64d687a96f12fe2cb638fc",
                Kind::RawSha256 => "4ed1458f0777e51e79e60cfbb201b9ba8dedd51e7cc0f1463dc6adac8174d8b3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/parent-join/parent-join-client-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "c5b288f797c7e7e06ce2230d7acfc5bbb3faae0e1f19eb53b7f85675a05a16a7",
                Kind::JarClassV1 => "24f28b07f5aec5a739d14b2c3a825e92685e75a301040a96bdcbd9fb8185f4d4",
                Kind::JarMavenCentralV1 => "e2470f87f524d2fd6f2772cb126b95da6ae29c8c",
                Kind::JarRawV1 => "c13ee79fb7ac0aeb5f6767173fbc4f43aaed5035955cc342732d0122b1877d68",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/percolator/percolator-client-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "4dd6d980a2da4dd00e1e6b3b127e862656fc3bedd4fe8cd0c5dbe080c12890ce",
                Kind::JarMavenCentralV1 => "29e19505e6f00884eccad1d650fe72f637ffd1c1",
                Kind::RawSha256 => "8e7df7191cd0ba0ab80b7ea95581671dbbdcfec7ff67e7666a70e47c550cd40c",
                Kind::JarRawV1 => "1d99b76fc2c20a580c0058194dd907585d688236304424a7ee8c98515e89d261",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/rank-eval/rank-eval-client-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "f975e7111bf29236b388e2b47072c640715d137f7e7d320f94eda1a951218257",
                Kind::JarClassV1 => "564235e29238445483fc09203c00c8065ab7c2a65572b38283b20832ae338973",
                Kind::JarMavenCentralV1 => "ed34ef9dd766dbfaf574ac4962005e0fb954e553",
                Kind::RawSha256 => "ae3b002d5d41333da503e071a4b3767625ad5c866983d2a02e8a2933dc5a5a6d",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/commons-codec-1.15.jar",
            combined! {
                Kind::JarRawV1 => "54324aabfbe16b3cf75c314676de4524a6d2cf3250ac8ea9a61507fc8ff79bd3",
                Kind::JarClassV1 => "130c47d01c72e33a0332a95d4a1d8718a2465ac015ec6b3863174618caf58e2f",
                Kind::JarMavenCentralV1 => "49d94806b6e3dc933dacbd8acb0fdbab8ebd1e5d",
                Kind::RawSha256 => "b3e9f6d63a790109bf0d056611fbed1cf69055826defeb9894a71369d246ed63",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/commons-logging-1.1.3.jar",
            combined! {
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
                Kind::JarMavenCentralV1 => "f6f66e966c70a83ffbdb6f17a0919eaf7c8aca7f",
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/elasticsearch-rest-client-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
                Kind::JarMavenCentralV1 => "810c3578e58f02e112a918f589b153222891d41b",
                Kind::RawSha256 => "91cfab57383a284c826a6dd513dcfba309aea0b473af18582552c18c8b5dd078",
                Kind::JarRawV1 => "08b04d8bc63a2cc976bdad6b6b043d3f17e3c0437450ccea6be2c7316d806c8d",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/elasticsearch-ssl-config-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "ffa840a844f490041aa65bbc4763e3b74b926bcefb8f97eb19f665ee9b12b1d2",
                Kind::JarClassV1 => "bfa8378ae967a7d0e3dc1ffc52b65da2eb6afe95763c54dbc3e7c97f9169c5f6",
                Kind::JarMavenCentralV1 => "27e77cd3bb6f7b8a6651a157ec3f18147a476d33",
                Kind::RawSha256 => "b2b9e1eeb2c446a19c4282d941aba1c26f453959bde71c33f779546923182c22",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/httpasyncclient-4.1.4.jar",
            combined! {
                Kind::RawSha256 => "50e981a8e567a16ebdad104605b156540a863459fa127b8ba647f310dfc83ef8",
                Kind::JarRawV1 => "45f280f8ed6c9600a41afc2bf6f73dbd96bab37c0cd0bf66e52f916c9c57a9a4",
                Kind::JarClassV1 => "77f6448014b48e685332f32f0aec9c34383bb7a69ea8e2586f2a738418be6669",
                Kind::JarMavenCentralV1 => "f3a3240681faae3fa46b573a4c7e50cec9db0d86",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/httpclient-4.5.14.jar",
            combined! {
                Kind::JarRawV1 => "ce7270d53369efb990e03e5075b51f9c732fcb088f9cb55eda788cdf46aa69a6",
                Kind::RawSha256 => "c8bc7e1c51a6d4ce72f40d2ebbabf1c4b68bfe76e732104b04381b493478e9d6",
                Kind::JarClassV1 => "a539ce4f6e2cb59c8b36372e5951db869d2eaf8f45f04eae72255fb9f38c6f16",
                Kind::JarMavenCentralV1 => "1194890e6f56ec29177673f2f12d0b8e627dec98",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/httpcore-4.4.12.jar",
            combined! {
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::JarMavenCentralV1 => "21ebaf6d532bc350ba95bd81938fa5f0e511c132",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/httpcore-nio-4.4.12.jar",
            combined! {
                Kind::RawSha256 => "11448f4b5c7f13d9396a67b33aa938d05f660665e0f14fd08e25acfd3c20ae80",
                Kind::JarRawV1 => "24e4f97e5ff3f463dc40dbdd72e84f11c506e479c31e5a966dc92f68367ef631",
                Kind::JarClassV1 => "46e99f783b2e361033e0b5d74193643715b4faa2037d6aaf39aa1bf93fc05adf",
                Kind::JarMavenCentralV1 => "84cd29eca842f31db02987cfedea245af020198b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/reindex/reindex-client-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "5179d66d44a1049c11043ac0063c3074df838479dde9345dc45e2235b58f265f",
                Kind::RawSha256 => "16cad00497cfd6774128943c04a54c160ba6a3f6d7d7711256e1c1cada75a926",
                Kind::JarClassV1 => "711a6e126ab14e65b90a7bce7adb29004fbb365afe7563aa2160bd47c43cb2ef",
                Kind::JarMavenCentralV1 => "a614744ddcfb609b4601e9c42cb226b2f0f9cd44",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/repositories-metering-api/repositories-metering-api-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "e60896c765664d3ce70ed924a68bceafff2d7bc7cb55c1e718e060ef7891ae80",
                Kind::JarRawV1 => "ed495bb186e597b698bfe701d3733192061450d89594a6a312ce26f4aaec13ea",
                Kind::JarClassV1 => "d34b3d50656d42f4885c86d33b78b46d07f069207233a782e8989e7d5312c533",
                Kind::JarMavenCentralV1 => "c9537a772b756439f77e9248cd38d82ff2205473",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/repository-encrypted/repository-encrypted-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "cef5f67b32aaa97c9f9c3a91cf1fdbfcc2d3da385d8761857498a5695fc19e58",
                Kind::JarMavenCentralV1 => "18fd112805343ac6f6e4c0242b52d779df208ea8",
                Kind::JarRawV1 => "4155d96bb1d4df35f566beebe24cac613c472e000ae6e9376b6afa0f86d2d0ca",
                Kind::RawSha256 => "5e4bbbb58d10544eed26cb292d99d5257453359e02c8753f18896e9ec67b96ed",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/repository-url/commons-codec-1.15.jar",
            combined! {
                Kind::JarRawV1 => "54324aabfbe16b3cf75c314676de4524a6d2cf3250ac8ea9a61507fc8ff79bd3",
                Kind::JarClassV1 => "130c47d01c72e33a0332a95d4a1d8718a2465ac015ec6b3863174618caf58e2f",
                Kind::JarMavenCentralV1 => "49d94806b6e3dc933dacbd8acb0fdbab8ebd1e5d",
                Kind::RawSha256 => "b3e9f6d63a790109bf0d056611fbed1cf69055826defeb9894a71369d246ed63",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/repository-url/commons-logging-1.1.3.jar",
            combined! {
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
                Kind::JarMavenCentralV1 => "f6f66e966c70a83ffbdb6f17a0919eaf7c8aca7f",
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/repository-url/httpclient-4.5.14.jar",
            combined! {
                Kind::RawSha256 => "c8bc7e1c51a6d4ce72f40d2ebbabf1c4b68bfe76e732104b04381b493478e9d6",
                Kind::JarClassV1 => "a539ce4f6e2cb59c8b36372e5951db869d2eaf8f45f04eae72255fb9f38c6f16",
                Kind::JarMavenCentralV1 => "1194890e6f56ec29177673f2f12d0b8e627dec98",
                Kind::JarRawV1 => "ce7270d53369efb990e03e5075b51f9c732fcb088f9cb55eda788cdf46aa69a6",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/repository-url/httpcore-4.4.12.jar",
            combined! {
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::JarMavenCentralV1 => "21ebaf6d532bc350ba95bd81938fa5f0e511c132",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/repository-url/log4j-1.2-api-2.17.1.jar",
            combined! {
                Kind::JarRawV1 => "b6fa23d8eb7cad6b08c18f318a869f85778b0cdc9e9eac56a9fde7db263fdd72",
                Kind::RawSha256 => "ca3e9150f95c31d15b9680a609b8817f8549bd395591c5ca55957d1ef0f464d6",
                Kind::JarClassV1 => "6e705c0891b1027df1444ecf13cce77d48cfa4b039bf2fbb596937a2f41bdfc1",
                Kind::JarMavenCentralV1 => "db3a7e7f07e878b92ac4a8f1100bee8325d5713a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/repository-url/repository-url-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "cea3ef483be29377186d52fdb1a2300a53cb2a59fe6f046a5744b9e6a304ee18",
                Kind::JarMavenCentralV1 => "34ca2278848c18a21a1316a9892cb76b7b6bf74e",
                Kind::RawSha256 => "c07754568414d26e9331006a284417f9b300626170a90218c8f7448965a42d25",
                Kind::JarRawV1 => "540f1fe347ed8580437abc061b72999413a8d6d17a5c031840f91b5f4c4d3c7b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/runtime-fields-common/elasticsearch-dissect-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "4b0570e1077894effaac3b5efde9bc9b03e15e0ef88d9a58df99f1ba476020cd",
                Kind::RawSha256 => "63c18fa87875fab72bd469449040bb9884204414557e5f6ffdcb8da9cfa98e87",
                Kind::JarClassV1 => "95041177b60ad6b80438591597dd00a08188314bfd02b35a7ece3f2ec9b37534",
                Kind::JarMavenCentralV1 => "11d3441c1516d8960fed65539b985f4ad274b572",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/runtime-fields-common/elasticsearch-grok-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "fee1097201de7908879f65cdaf0fc54bc60a4e023450ca1f9eaf3ffb7b14bc84",
                Kind::JarMavenCentralV1 => "69afea1244752b8cd4b212e3c942e4a1ec3191c6",
                Kind::JarRawV1 => "e6b6ef74174dec6b1c50d3cb6bf82d73413e56d9d038ddc7e06a26e1eb6fea14",
                Kind::RawSha256 => "a2267aef94e0a815521d5cb233aacb512719cd96721f64f056dbc085397f761e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/runtime-fields-common/jcodings-1.0.44.jar",
            combined! {
                Kind::RawSha256 => "49190d6ad09056de57d7ed41ed5b4b105e033557b5dd170702decdcf05ee341a",
                Kind::JarClassV1 => "4dd715e0832222e95234af07d22995a0cc40fdc7328ec88b355877d060650532",
                Kind::JarMavenCentralV1 => "a6884b2fd8fd9a56874db05afaa22435043a2e3e",
                Kind::JarRawV1 => "3ff823499e3d4fd4815efbf4a54311290f5a64592d944e901f8a2940009b35dd",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/runtime-fields-common/joni-2.1.29.jar",
            combined! {
                Kind::RawSha256 => "aa4b71415682f3d7fa44083cd94a9ec48478ec3b9c30947b4152913d41b1004d",
                Kind::JarRawV1 => "5f5c1894e9780c6e408de3baeee7585886d8fbc3ae73b6af25722912567cdcff",
                Kind::JarClassV1 => "e9629b51e39e64cd10c5a7a7e10dbb0c5428ce4006a864da5fba4e1a1ec03ff2",
                Kind::JarMavenCentralV1 => "3cb751702e1194ff24259155db4d37e9383d4320",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/runtime-fields-common/runtime-fields-common-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "89d87db38e90ac8644866fff394a287f3532deed42bcae70a3fcfacc3c6bc842",
                Kind::JarClassV1 => "9d323f4300e50014348a54c6ff6c8177ac919c10cbc1267edc7c624b83ef937a",
                Kind::JarMavenCentralV1 => "11cd7428c0107583e1e8e67955783e5eedb297b3",
                Kind::RawSha256 => "aacdd217d989390c49898a4d6282c6b845b9afcdd2f02d8f31565c6a8f6a950a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/search-business-rules/search-business-rules-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "a63ac077e612d066d2f724bf9a0fcba7eea3f3fe88e775749910971ea17da709",
                Kind::JarRawV1 => "430c4d7ba262c41e23d0fad61bd30ac514a2c39a484a62cfdcf2f74b90e65c8c",
                Kind::JarClassV1 => "7aa97a9c72b5be6a324790f600c9ea8dba51f900d767431e2ffdc7915ff67118",
                Kind::JarMavenCentralV1 => "26c46e001a0ae78da82761111f9baed18e860e9e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/searchable-snapshots/preallocate-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "99dd11c66ab352f003adcdb42a2fede7cc91d196ca14c7f24faed294f527a819",
                Kind::JarClassV1 => "195c7a82e8677613fa70a43488c101b19df6cf185009b4ea3733e653d41e4c18",
                Kind::JarMavenCentralV1 => "d00aaca4552d12530ddc7264f18b23aec17df916",
                Kind::RawSha256 => "3a4cfe79e95b2973d195627f924f3c7d7ebefdeb0b1d01bb7595eeb5d023ac2c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/searchable-snapshots/searchable-snapshots-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "da2cd1b6483d9e8d3777a8691ae8bb8a4e66929f178b8e43d3cecf78d4762b62",
                Kind::JarRawV1 => "e6fc5cee35d0391364d8a6e81d6dded9e6609e5928a9d90433842526cf9c32fc",
                Kind::JarClassV1 => "2229b417978581cdacb23d199852d8486eed5a384b0c624d6224ed999a951007",
                Kind::JarMavenCentralV1 => "9fba210c2a1cfd05b637e3ace9afe9f610e30be7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/snapshot-repo-test-kit/snapshot-repo-test-kit-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "cc25371afdf8bdd1792cb03e7064dd74ed941d45d2976cc7c6f9bee35218cde2",
                Kind::JarClassV1 => "945a94d1d4802f58a962dcaf412ad291e34f55ed4d3e3e67847e6226c7e32d05",
                Kind::JarMavenCentralV1 => "372c5e30e0b6c109dc385f8d939bd181ba52f23c",
                Kind::JarRawV1 => "d97670316621606ca27ba18f99a78dd3ff1f05c56d414344d5762471ad812566",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/spatial/spatial-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "474a3b7995bbefb730cfa28ed3dc980659ed475c750a1f733438507785be46b0",
                Kind::JarMavenCentralV1 => "f582e1cb988b096f68728aa9db138c489767a4b7",
                Kind::RawSha256 => "f02cfc27bc8d638a5ce70bd39c1aab89631908162c4e894eff71bbb3b921404a",
                Kind::JarRawV1 => "ca5e27b8632d6ffa4f128a30153d21ef569f401b4c086de067a9159ec6a4000c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transform/transform-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "6064123301cc9c6deab947107e7f302291c943c54679cd01a644a58478984bdf",
                Kind::JarClassV1 => "82d76623eaa33d324428dc095ea731db6d266b9e2f3c571a03eadf804b3fce20",
                Kind::JarMavenCentralV1 => "ff41d71df1b473d86ac4f29e12162d1dd309a641",
                Kind::JarRawV1 => "045517d9b79f2617c04ea65cb77e6c29cf6b5305c6b28aa0045a95d4c17cbb14",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/netty-buffer-4.1.94.Final.jar",
            combined! {
                Kind::RawSha256 => "8066ee7c49f9f29da96ee62f7cb13bee022cb4b68e51437b33da3b6d01398f13",
                Kind::JarRawV1 => "2b95b6a9a4099829f458d1119332939b6ff8269c49ece4d8f55e4dc3a8060d8a",
                Kind::JarClassV1 => "4edebb866fd1d2ada39996f6aa81032948c4890a009b7fd2c8d8fee6ebb0dc94",
                Kind::JarMavenCentralV1 => "eec248b26f16e888688e5bb37b7eeda76b78d2f7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/netty-codec-4.1.94.Final.jar",
            combined! {
                Kind::JarClassV1 => "cba835a85d54eb82d0f3689a0023852723a1534797e21fbcf4576851ac88980e",
                Kind::JarMavenCentralV1 => "c70ef20ca338558147887df60f46341bc47f6900",
                Kind::RawSha256 => "91243776ad68b4d8e39eafb9ec115e1b8fa9aecd147b12ef15bb691639498328",
                Kind::JarRawV1 => "140023768eff45864225f5faf790a653340067ad005fdd1a3a3c86969378d32c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/netty-codec-http-4.1.94.Final.jar",
            combined! {
                Kind::JarRawV1 => "ba79f3948d040f62f54e0f3ff230fa42be35f445e72e4df0e8dd2e46b72c5e2e",
                Kind::RawSha256 => "1ada4580f68cd17a534fb3c0337087073223a76cb77304dbe5a1b19df3d53c2f",
                Kind::JarClassV1 => "8857d55f7993e6d8f19cac137321cc3aa9850e9650129b25e769fefe3d050c46",
                Kind::JarMavenCentralV1 => "9e5404764092c1f6305ad5719078f46ab228d587",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/netty-common-4.1.94.Final.jar",
            combined! {
                Kind::JarClassV1 => "df35e6272db29383a70a30283fe01d0ef17b21d93411e987724497caf76f4788",
                Kind::JarMavenCentralV1 => "ad4ecf779ebc794cd351f57792f56ea01387b868",
                Kind::RawSha256 => "cb8d84a3e63aea90d0d7a333a02e50ac751d2b05db55745d981b5eff893f647b",
                Kind::JarRawV1 => "c13992db79a3d58e376dbf1569022b9e24e31af5dbfacb74e37b67990421dd70",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/netty-handler-4.1.94.Final.jar",
            combined! {
                Kind::JarRawV1 => "fa3a5918066bfa89e8e098e87b5badafe9e8bcfb5844670f0d9c29de799ad72c",
                Kind::RawSha256 => "8e50719a9ab89e33ef85c5f36d780e0d7056b3f768b07d261d87baed7094eb3c",
                Kind::JarClassV1 => "10b94303a43710544b34e81e5654aa67f4d42496483f72cb832043a53ba245da",
                Kind::JarMavenCentralV1 => "cd9121ce24d6d3f2898946d04b0ef3ec548b00b4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/netty-resolver-4.1.94.Final.jar",
            combined! {
                Kind::JarRawV1 => "ff2173856cfff7a6ec7e27ce82713140f62f74bb93708a8efa25ca61d7263099",
                Kind::RawSha256 => "bd26e9bc5e94e2d3974a93fdf921658eff4f033bfd4c5208607760ab54298617",
                Kind::JarClassV1 => "2b9c9a2398ae25cdb42d10c7a68470cb4acecb48a512fd93e7780d76ff5ab6f0",
                Kind::JarMavenCentralV1 => "e96f649e8e9dcb29a1f8e95328b99c9eb6cf76c2",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/netty-transport-4.1.94.Final.jar",
            combined! {
                Kind::JarClassV1 => "13238f2611717f6bdff7a9e78b76f35a07953b4912f017e79ad0482da577f4ac",
                Kind::JarMavenCentralV1 => "ec783a737f96991a87b1d5794e2f9eb2024d708a",
                Kind::JarRawV1 => "29071fb08fb276b13b55127deb4e9c9812375a693e17583799f853be44c4b6df",
                Kind::RawSha256 => "a75afa84ca35a50225991b39e6b6278186e612f7a2a0c0e981de523aaac516a4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/netty-transport-native-unix-common-4.1.94.Final.jar",
            combined! {
                Kind::JarRawV1 => "bc816575ecf9fb2e8ca92608613588d6fcb9979a57f0274a54bae84206ed015a",
                Kind::RawSha256 => "27d0dff1cd743190279becacfb372fe4d45b266edafad9f1c6c01b04d00280eb",
                Kind::JarClassV1 => "ebbcae47c82b99f681b14c380a27ee969a25a78987818957fa79b20510565ebe",
                Kind::JarMavenCentralV1 => "3fa5f9d04b6b782d869d6e0657d896eeadca5866",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/transport-netty4/transport-netty4-client-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "41bf923024322ed5f877589c10aef34e8558fddf842fb0199ec39bac9d26a571",
                Kind::RawSha256 => "abeaef42fa472c90caa8f30c585b228b4ee587b135161fb3083ac054ce331c23",
                Kind::JarClassV1 => "a52276dc5b72c095f2b67d8fe0fc36a4eb8410d8976638526b8bac686619f2c3",
                Kind::JarMavenCentralV1 => "87bb2f8d6b0f8f0e1d92f58cc8656a831d07a0f6",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/unsigned-long/unsigned-long-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "a3dead2cd7168fee5c28dc4b167738bc866fe083f0e9af1fd060ed3e7122c368",
                Kind::JarClassV1 => "883cbd7ea3f95a6c1d2031d36d39a5b7913685573d66e8d212b36a505636f525",
                Kind::JarMavenCentralV1 => "5448a5644917cb57d56f03ae07c0f2e01e067f05",
                Kind::JarRawV1 => "b934d9df7fdb7435c62b115f982f572a93f35f2d0801591e4754683cbf0804c8",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/vector-tile/log4j-slf4j-impl-2.17.1.jar",
            combined! {
                Kind::RawSha256 => "e9a03720e5d5076009c2530635da9d08485e28a0b0ec20708dadc51afb78e41e",
                Kind::JarRawV1 => "a78204ec2fbf25286ad7f7c7d497843d9758c49b55895c8a413998b74a10428e",
                Kind::JarClassV1 => "9ad437ba29a9f5f245d4ccd6fc067aaa9a4e02dc7c677d61ffbc395738971862",
                Kind::JarMavenCentralV1 => "84692d456bcce689355d33d68167875e486954dd",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/vector-tile/mapbox-vector-tile-3.1.0.jar",
            combined! {
                Kind::RawSha256 => "e2bb4c89fae43c6057a88f5d3b11256e78a255467e31654672d72c7b228de435",
                Kind::JarRawV1 => "18ee573e68b4620946d02a130f31718abaabf7d9e311420970fc00690aae1472",
                Kind::JarClassV1 => "77f4b7af48f79b5f302a9b678a1b7de61910d51184c4987c811e31d1ffdbbc02",
                Kind::JarMavenCentralV1 => "06c4432c7885a3938571a57e73cc1444d7a39f12",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/vector-tile/protobuf-java-3.16.3.jar",
            combined! {
                Kind::RawSha256 => "6d3758b483e1ac7505649371a5b058a717260ef470100fefad7a1e2aba75e06f",
                Kind::JarRawV1 => "a799f755a999f87673ef10c5a17ecafe24920258e584fbe4906a7c5d8f4835f0",
                Kind::JarClassV1 => "9a1a7d0c778a3151df6b08b5f9a705e8f558c611d4a2813d655f561bd70aad31",
                Kind::JarMavenCentralV1 => "0c5a5719be44c6cd94e478bfe17c4cbe4b0948f4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/vector-tile/slf4j-api-1.6.2.jar",
            combined! {
                Kind::RawSha256 => "c26413df1741466d8bec549a4b7f6480acb2ac928b156be8c5e3feaf9cba1b59",
                Kind::JarRawV1 => "51bf440ef88e6f86d2bb6097c537b5afaf8ca93a7fe4a3ee1a203113a6f279d3",
                Kind::JarClassV1 => "af24a6f8c98dd2a81bfbfa9503452c31b64900b76e33a258e102086f661c1133",
                Kind::JarMavenCentralV1 => "8619e95939167fb37245b5670135e4feb0ec7d50",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/vector-tile/vector-tile-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "5ff0aee527ca41e2b7e0006d5a8945738a0942929d531228c0cde3b9dff9de6d",
                Kind::JarRawV1 => "9fa1674135c6ec410621b3e3085c6cd6f536c847008544d70e9463b8088ca585",
                Kind::JarClassV1 => "819c44ad864fb1025e758d906f22347a3901006b691a26cf83a05330a2917a79",
                Kind::JarMavenCentralV1 => "9e0726dc87bb4f6a3826ae9562a1acacfcd158d0",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/vectors/vectors-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "3bab66be81fe77a01454c7b6494ea07b1476cf0b9e6e7b8f5e662ecb8fbe9d5f",
                Kind::JarRawV1 => "d2ebcdeb79a4523db9aed8daee41b1f76616fc213e8c9ad03400361aa89cbe0c",
                Kind::JarClassV1 => "bbb183f5d386de9cb701906839429fe12fa110ab45c1a3e08a4b94be74d1c3b6",
                Kind::JarMavenCentralV1 => "6572204aad68900c94b1728f6d9c63c01fb0f503",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/wildcard/wildcard-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "0f24d88d123d1e4d267dad0d982a724e133670800f0790867a21cbb7b7d53401",
                Kind::JarClassV1 => "4dd4aedc4cb4264c4c904696dca9786a4902c0c988579977b14d308e40d348e2",
                Kind::JarMavenCentralV1 => "0c490930058c1698e44a08cd573bea5a90066a4a",
                Kind::RawSha256 => "accdf5977d78004ac9e4ffafa3186e1d38c36d9021401b5800f950a123abe114",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-aggregate-metric/x-pack-aggregate-metric-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "93bcde8f57ac5767cfd4411d06c5318567dde903320932ca43373fbd1579eee0",
                Kind::JarClassV1 => "13cc85a8acc799fe2c6333cc974ef2cd1e011ca6dd89f4c59297e326543c9e8b",
                Kind::JarMavenCentralV1 => "d6afbd0713966261aea614ce4dad0cd93ffa3f34",
                Kind::RawSha256 => "fa27f1d6d65196bf03a6cec6c31a689e6951427e25ea8214b03a52a2c71ef03f",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-analytics/commons-math3-3.6.1.jar",
            combined! {
                Kind::JarClassV1 => "6bf92ce1f74505c615e5224765586998b0f9aecfa2d790479cd668f28d7101dd",
                Kind::JarMavenCentralV1 => "e4ba98f1d4b3c80ec46392f25e094a6a2e58fcbf",
                Kind::JarRawV1 => "de8a86e786ccdfae373551eee29b07428acdc7ac9eeb495f5b23fed90f36ff35",
                Kind::RawSha256 => "1e56d7b058d28b65abd256b8458e3885b674c1d588fa43cd7d1cbb9c7ef2b308",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-analytics/x-pack-analytics-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "dc358155cf56d1bb2043a6da08440940b068540f9de18abbe5ad4a8984283bb7",
                Kind::JarRawV1 => "9ca47c539c126a37b402555ccf27d906679dd42d52580fa848a068d8f3bbbe2b",
                Kind::JarClassV1 => "8db21035ab59196dedcfbf9a6096728086200cc0c755f4fa90d8124df2858a01",
                Kind::JarMavenCentralV1 => "d5e54a75ef0d5196a9723b3b25b908fd5eb29244",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-async/x-pack-async-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "6e0eec7f2d2d6b49f57719bdae004760dae874521624b0e5351bcfedb41d4766",
                Kind::JarMavenCentralV1 => "db1ccc542c116adcfa7a8937419f00b38873da85",
                Kind::JarRawV1 => "1d44351bd37331476ec587aa14ff78b5dceb8e75252a84e05696c3da4d4da44b",
                Kind::RawSha256 => "bc653c9c94278bba57996dce37fc11e58918f9f5c6a10542325f5d19bfa601ec",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-async-search/x-pack-async-search-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "06ea3b05b1d3901d0fad3d5794c005d0949273cacf18404ba1cc3428628d31c9",
                Kind::RawSha256 => "3923e29afe69043ad6fcd2fc959f3382705669a1f137e9dfac67ccfaa1dbe569",
                Kind::JarClassV1 => "e079e35651e243360c05f52194dbbef2493b3d68a9f0fa74fbf20b490e4518ac",
                Kind::JarMavenCentralV1 => "87be10231ee6ef7a108319a3cad88b9665c5dfff",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-autoscaling/x-pack-autoscaling-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "29272e1b63f1ff5ec9d3974f30a51a89a5ff708ccb7bdc42e37875d261e84d0a",
                Kind::JarRawV1 => "ce64464e43a4e739a9fd6461548256b477c9349023478b6ff830b389026d5133",
                Kind::JarClassV1 => "731ed7d14dfd6cd3fade80480d7659e5e2dfa97617b1bb6112fc66db390c9b55",
                Kind::JarMavenCentralV1 => "8ae38c522449a7c06e86f4839877b142d53c2b13",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ccr/x-pack-ccr-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "2879c14f15326ef71704c2d255d94d9f2bbe0632b2719ffc1139758af7f7f150",
                Kind::JarRawV1 => "9239e38adcf3477c883d532a2b480d5c485068032bf4c9a6861fb3e93738cf75",
                Kind::JarClassV1 => "b7d6463027007e5f5d2edc7bf1b1c8757c49038979189d138783d2acbb002f41",
                Kind::JarMavenCentralV1 => "9e88adcf6846fddc49b8f48cef8ea2924df17c2f",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/commons-codec-1.15.jar",
            combined! {
                Kind::RawSha256 => "b3e9f6d63a790109bf0d056611fbed1cf69055826defeb9894a71369d246ed63",
                Kind::JarRawV1 => "54324aabfbe16b3cf75c314676de4524a6d2cf3250ac8ea9a61507fc8ff79bd3",
                Kind::JarClassV1 => "130c47d01c72e33a0332a95d4a1d8718a2465ac015ec6b3863174618caf58e2f",
                Kind::JarMavenCentralV1 => "49d94806b6e3dc933dacbd8acb0fdbab8ebd1e5d",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/commons-logging-1.1.3.jar",
            combined! {
                Kind::RawSha256 => "70903f6fc82e9908c8da9f20443f61d90f0870a312642991fe8462a0b9391784",
                Kind::JarClassV1 => "35d9396d32fb07b599fc1e3a43f74cfc024b048592a88d7dab26e65a143322a6",
                Kind::JarMavenCentralV1 => "f6f66e966c70a83ffbdb6f17a0919eaf7c8aca7f",
                Kind::JarRawV1 => "4f7531b4462baf4a8a39ec7199fa80e9089017e8df2fc2aff9ad9fc8ed90f871",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/elasticsearch-nio-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "59e545296245fa7bd02c59addae9d6838dbf9d617a66e4b6e008709284394633",
                Kind::JarMavenCentralV1 => "2cf273cbfecd6768a6c5f5d043381f52b5c5b92f",
                Kind::RawSha256 => "8ff281f6daed1e9e4027f6f96336b499883c5fe7139aa391bd101cc3b2379e58",
                Kind::JarRawV1 => "4761326d092c86004ebe1ed54cd09020915554c6842b813da14797433ac8ec66",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/elasticsearch-ssl-config-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "ffa840a844f490041aa65bbc4763e3b74b926bcefb8f97eb19f665ee9b12b1d2",
                Kind::JarClassV1 => "bfa8378ae967a7d0e3dc1ffc52b65da2eb6afe95763c54dbc3e7c97f9169c5f6",
                Kind::JarMavenCentralV1 => "27e77cd3bb6f7b8a6651a157ec3f18147a476d33",
                Kind::RawSha256 => "b2b9e1eeb2c446a19c4282d941aba1c26f453959bde71c33f779546923182c22",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/httpasyncclient-4.1.4.jar",
            combined! {
                Kind::JarRawV1 => "45f280f8ed6c9600a41afc2bf6f73dbd96bab37c0cd0bf66e52f916c9c57a9a4",
                Kind::RawSha256 => "50e981a8e567a16ebdad104605b156540a863459fa127b8ba647f310dfc83ef8",
                Kind::JarClassV1 => "77f6448014b48e685332f32f0aec9c34383bb7a69ea8e2586f2a738418be6669",
                Kind::JarMavenCentralV1 => "f3a3240681faae3fa46b573a4c7e50cec9db0d86",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/httpclient-4.5.14.jar",
            combined! {
                Kind::JarRawV1 => "ce7270d53369efb990e03e5075b51f9c732fcb088f9cb55eda788cdf46aa69a6",
                Kind::JarClassV1 => "a539ce4f6e2cb59c8b36372e5951db869d2eaf8f45f04eae72255fb9f38c6f16",
                Kind::JarMavenCentralV1 => "1194890e6f56ec29177673f2f12d0b8e627dec98",
                Kind::RawSha256 => "c8bc7e1c51a6d4ce72f40d2ebbabf1c4b68bfe76e732104b04381b493478e9d6",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/httpcore-4.4.12.jar",
            combined! {
                Kind::JarClassV1 => "533767ad9f53f96e67b0127cefe8aac02d5879ed900bb1d9fa412e06e1f5de07",
                Kind::JarMavenCentralV1 => "21ebaf6d532bc350ba95bd81938fa5f0e511c132",
                Kind::RawSha256 => "ab765334beabf0ea024484a5e90a7c40e8160b145f22d199e11e27f68d57da08",
                Kind::JarRawV1 => "d9577e26a79c8ea57f228e361d90190bd3b4897e6d29faea4bf12649b50ab690",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/httpcore-nio-4.4.12.jar",
            combined! {
                Kind::JarClassV1 => "46e99f783b2e361033e0b5d74193643715b4faa2037d6aaf39aa1bf93fc05adf",
                Kind::JarMavenCentralV1 => "84cd29eca842f31db02987cfedea245af020198b",
                Kind::JarRawV1 => "24e4f97e5ff3f463dc40dbdd72e84f11c506e479c31e5a966dc92f68367ef631",
                Kind::RawSha256 => "11448f4b5c7f13d9396a67b33aa938d05f660665e0f14fd08e25acfd3c20ae80",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/log4j-1.2-api-2.17.1.jar",
            combined! {
                Kind::JarRawV1 => "b6fa23d8eb7cad6b08c18f318a869f85778b0cdc9e9eac56a9fde7db263fdd72",
                Kind::RawSha256 => "ca3e9150f95c31d15b9680a609b8817f8549bd395591c5ca55957d1ef0f464d6",
                Kind::JarClassV1 => "6e705c0891b1027df1444ecf13cce77d48cfa4b039bf2fbb596937a2f41bdfc1",
                Kind::JarMavenCentralV1 => "db3a7e7f07e878b92ac4a8f1100bee8325d5713a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/netty-buffer-4.1.94.Final.jar",
            combined! {
                Kind::JarClassV1 => "4edebb866fd1d2ada39996f6aa81032948c4890a009b7fd2c8d8fee6ebb0dc94",
                Kind::JarMavenCentralV1 => "eec248b26f16e888688e5bb37b7eeda76b78d2f7",
                Kind::RawSha256 => "8066ee7c49f9f29da96ee62f7cb13bee022cb4b68e51437b33da3b6d01398f13",
                Kind::JarRawV1 => "2b95b6a9a4099829f458d1119332939b6ff8269c49ece4d8f55e4dc3a8060d8a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/netty-codec-4.1.94.Final.jar",
            combined! {
                Kind::JarClassV1 => "cba835a85d54eb82d0f3689a0023852723a1534797e21fbcf4576851ac88980e",
                Kind::JarMavenCentralV1 => "c70ef20ca338558147887df60f46341bc47f6900",
                Kind::RawSha256 => "91243776ad68b4d8e39eafb9ec115e1b8fa9aecd147b12ef15bb691639498328",
                Kind::JarRawV1 => "140023768eff45864225f5faf790a653340067ad005fdd1a3a3c86969378d32c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/netty-codec-http-4.1.94.Final.jar",
            combined! {
                Kind::RawSha256 => "1ada4580f68cd17a534fb3c0337087073223a76cb77304dbe5a1b19df3d53c2f",
                Kind::JarClassV1 => "8857d55f7993e6d8f19cac137321cc3aa9850e9650129b25e769fefe3d050c46",
                Kind::JarMavenCentralV1 => "9e5404764092c1f6305ad5719078f46ab228d587",
                Kind::JarRawV1 => "ba79f3948d040f62f54e0f3ff230fa42be35f445e72e4df0e8dd2e46b72c5e2e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/netty-common-4.1.94.Final.jar",
            combined! {
                Kind::RawSha256 => "cb8d84a3e63aea90d0d7a333a02e50ac751d2b05db55745d981b5eff893f647b",
                Kind::JarRawV1 => "c13992db79a3d58e376dbf1569022b9e24e31af5dbfacb74e37b67990421dd70",
                Kind::JarClassV1 => "df35e6272db29383a70a30283fe01d0ef17b21d93411e987724497caf76f4788",
                Kind::JarMavenCentralV1 => "ad4ecf779ebc794cd351f57792f56ea01387b868",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/netty-handler-4.1.94.Final.jar",
            combined! {
                Kind::JarRawV1 => "fa3a5918066bfa89e8e098e87b5badafe9e8bcfb5844670f0d9c29de799ad72c",
                Kind::JarClassV1 => "10b94303a43710544b34e81e5654aa67f4d42496483f72cb832043a53ba245da",
                Kind::JarMavenCentralV1 => "cd9121ce24d6d3f2898946d04b0ef3ec548b00b4",
                Kind::RawSha256 => "8e50719a9ab89e33ef85c5f36d780e0d7056b3f768b07d261d87baed7094eb3c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/netty-resolver-4.1.94.Final.jar",
            combined! {
                Kind::RawSha256 => "bd26e9bc5e94e2d3974a93fdf921658eff4f033bfd4c5208607760ab54298617",
                Kind::JarRawV1 => "ff2173856cfff7a6ec7e27ce82713140f62f74bb93708a8efa25ca61d7263099",
                Kind::JarClassV1 => "2b9c9a2398ae25cdb42d10c7a68470cb4acecb48a512fd93e7780d76ff5ab6f0",
                Kind::JarMavenCentralV1 => "e96f649e8e9dcb29a1f8e95328b99c9eb6cf76c2",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/netty-transport-4.1.94.Final.jar",
            combined! {
                Kind::JarRawV1 => "29071fb08fb276b13b55127deb4e9c9812375a693e17583799f853be44c4b6df",
                Kind::JarClassV1 => "13238f2611717f6bdff7a9e78b76f35a07953b4912f017e79ad0482da577f4ac",
                Kind::JarMavenCentralV1 => "ec783a737f96991a87b1d5794e2f9eb2024d708a",
                Kind::RawSha256 => "a75afa84ca35a50225991b39e6b6278186e612f7a2a0c0e981de523aaac516a4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/netty-transport-native-unix-common-4.1.94.Final.jar",
            combined! {
                Kind::RawSha256 => "27d0dff1cd743190279becacfb372fe4d45b266edafad9f1c6c01b04d00280eb",
                Kind::JarRawV1 => "bc816575ecf9fb2e8ca92608613588d6fcb9979a57f0274a54bae84206ed015a",
                Kind::JarClassV1 => "ebbcae47c82b99f681b14c380a27ee969a25a78987818957fa79b20510565ebe",
                Kind::JarMavenCentralV1 => "3fa5f9d04b6b782d869d6e0657d896eeadca5866",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/transport-netty4-client-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "41bf923024322ed5f877589c10aef34e8558fddf842fb0199ec39bac9d26a571",
                Kind::JarClassV1 => "a52276dc5b72c095f2b67d8fe0fc36a4eb8410d8976638526b8bac686619f2c3",
                Kind::JarMavenCentralV1 => "87bb2f8d6b0f8f0e1d92f58cc8656a831d07a0f6",
                Kind::RawSha256 => "abeaef42fa472c90caa8f30c585b228b4ee587b135161fb3083ac054ce331c23",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/transport-nio-client-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "414b5707757d4e3d3f7bc094c65ddf8d6cc9f6428fadeaf5c214fc97754bf7fe",
                Kind::JarClassV1 => "1f3341bf5eeeb60daf30e663e14cde336688d1b1ecf18a1bfd3a55cc582bb640",
                Kind::JarMavenCentralV1 => "671ba0bf21f7747111de1ed72fd85bf935b5c90e",
                Kind::JarRawV1 => "4d653eb51c43577b0c11077b91e0c8b488f2b5b5f8fdc1dc8e8317bfca0edd1e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/unboundid-ldapsdk-4.0.8.jar",
            combined! {
                Kind::JarRawV1 => "c8aad0122c5a74a6831693beb2565ab24feea8ceb8b31b2e19d578a3f53526ec",
                Kind::JarClassV1 => "48e4718c28e7303c0922cc86b6508d69ed84e225dd6b12ec2fb76ee5cd390172",
                Kind::JarMavenCentralV1 => "bf1a0d3790f8f7bd28f1172323c26fed2e3bbaa5",
                Kind::RawSha256 => "de2e896ab2989dee25f67fe2a3aaf789da0ae29e33aa86c9a359acb5bf9ca1fd",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-core/x-pack-core-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "3fa7623dfb6c7b249590851847bf294736166b7fcf0573137b53e050a42be52d",
                Kind::JarRawV1 => "ce900995b7f3b708a6dc0d7191fd1e3c7068ab9f9c5a676018cc88260f789f20",
                Kind::JarClassV1 => "26ff5bcd4bd2f73f4919f8bc6143fe32a3ee9ecc28928bea0cdd3b9b0e490fa5",
                Kind::JarMavenCentralV1 => "ff5e5747d18f29d8466ca4b8250dc439b4c644e9",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-data-streams/x-pack-data-streams-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "699d78d2ac7211ea4197b47469aac438d520d11a1defa725dd10e395ac6049c2",
                Kind::JarClassV1 => "5edc3dc064069cd5f5d9b2c57bb5665ad9e6c47a4f64f755a705f79d758e42cc",
                Kind::JarMavenCentralV1 => "af60a90d8fc040053a66b437436db7d87fc5a935",
                Kind::RawSha256 => "df185240296d66e92739b14c231ae045d635b0f6e008a22b10294bd0903df1a0",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-deprecation/elasticsearch-rest-client-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "91cfab57383a284c826a6dd513dcfba309aea0b473af18582552c18c8b5dd078",
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
                Kind::JarMavenCentralV1 => "810c3578e58f02e112a918f589b153222891d41b",
                Kind::JarRawV1 => "08b04d8bc63a2cc976bdad6b6b043d3f17e3c0437450ccea6be2c7316d806c8d",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-deprecation/elasticsearch-rest-client-sniffer-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "a1dd5002b52f0cc9b2f37176ce492036514df0e26fc693b0572a549d53969875",
                Kind::JarMavenCentralV1 => "e1731fdf0273e46d5f165f7b3c1db4c398d2b141",
                Kind::JarRawV1 => "a5a81a9ffd9a575102053cba386bac255c70e5ab03a94ae1173526512b658c46",
                Kind::RawSha256 => "6285c145746f0a9d0f7018fb9624fe518562cc5f6cebdf7030f6ad390c3d823c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-deprecation/x-pack-deprecation-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "4578d8b8520ca7609749fa1426eb43b9dccc4aa6c28a0ba27021e9b3a4144aa8",
                Kind::JarMavenCentralV1 => "d54acef422d8ca6d91003dae8593859f15f5adad",
                Kind::RawSha256 => "48b6a51f9cee8cf55ad3502ada4e2f10640c73e15898b88103771f08ac7ed1a9",
                Kind::JarRawV1 => "edb58343a51a5a2619d297ddd685330a7ada5e2ff846040c6215f4bce52432f3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-deprecation/x-pack-monitoring-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "ee5bf2c4c90beb2c806dfc7a061446115eaf485a7149e5c35e01f08670dc92d4",
                Kind::JarClassV1 => "b2eadff8f3958fb1020f8122f0754aa0f03b4326a3ba130a357223aa864c3d2e",
                Kind::JarMavenCentralV1 => "c87801621b55e004a701bbbd07f3c8b3f391f09b",
                Kind::RawSha256 => "d191f4f8e082bebbbed6743d4edc5c6ee1db569bbc838174bf5c9296ed68f5ea",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-enrich/elasticsearch-rest-client-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
                Kind::JarMavenCentralV1 => "810c3578e58f02e112a918f589b153222891d41b",
                Kind::RawSha256 => "91cfab57383a284c826a6dd513dcfba309aea0b473af18582552c18c8b5dd078",
                Kind::JarRawV1 => "08b04d8bc63a2cc976bdad6b6b043d3f17e3c0437450ccea6be2c7316d806c8d",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-enrich/reindex-client-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "16cad00497cfd6774128943c04a54c160ba6a3f6d7d7711256e1c1cada75a926",
                Kind::JarRawV1 => "5179d66d44a1049c11043ac0063c3074df838479dde9345dc45e2235b58f265f",
                Kind::JarClassV1 => "711a6e126ab14e65b90a7bce7adb29004fbb365afe7563aa2160bd47c43cb2ef",
                Kind::JarMavenCentralV1 => "a614744ddcfb609b4601e9c42cb226b2f0f9cd44",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-enrich/x-pack-enrich-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "fc5c2352505661c67fad848839475c06e5ad1cc152c1ba0e9f2452ee380c2cad",
                Kind::JarClassV1 => "0c40e9517e16b4643befa2541c95822960abda2b7d4ba2e9314d2426beccf5d2",
                Kind::JarMavenCentralV1 => "4029686202a4ba69b6997fee32f94bf948761865",
                Kind::JarRawV1 => "ca3dd314a602b5a5c5690acea8e5810d9e22db40f22ff9aa81bbd1c3b2e128c4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-eql/x-pack-eql-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "b07ed4f84c9ea621633d90462f35d56af85f5c1b5008c8bc119c13539bfadbe0",
                Kind::RawSha256 => "2194718c2620e2ec352ab5d82d1e9c57820e64d819be446f44d3f034e8b1104f",
                Kind::JarClassV1 => "4f0ee2ded9e5ef4104a36158d6c4b15c70b57235bd70dc412f8d8c27394cd4e0",
                Kind::JarMavenCentralV1 => "f5b2f50a4dbc9aea226afd83de86eee408382c12",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-fleet/x-pack-fleet-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "7d0b9b00ebba4fd2b66f1ca70dec6b63bb7f19d3c99f194538b118d968d0bca5",
                Kind::RawSha256 => "40edefcbe472032dace026e13001629d85c6d72f221a61e4b4ae0430ae047af3",
                Kind::JarClassV1 => "0d8f92a222b5b0877e32fdc5199c1514b94373f22dd58b063fde45a1cab95bea",
                Kind::JarMavenCentralV1 => "a1d0369ec50f65f06818ae98a32fd91133fb4370",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-graph/x-pack-graph-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "58fd765ebc75520cbb8eb4a37c9d5438a4c91d3c0f03865c66132ca090741ef9",
                Kind::JarClassV1 => "c68bc2f22ff07390e213dc94bc3c42587f56ba1f742653ecedf8e377863f32cc",
                Kind::JarMavenCentralV1 => "e162c749d4b1191ab8057b601ea122ab2489c555",
                Kind::RawSha256 => "853338214d2fd7a97f716ce3feefefe943878e4b675a18317a31b9b6ec5a4197",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/cryptacular-1.2.4.jar",
            combined! {
                Kind::JarClassV1 => "cc49125e2781a6a2e6fa1b2202d2f3ec7879ec0df717421e4108dd24b5d97309",
                Kind::JarMavenCentralV1 => "4994c015d87886212683245d13e87f6fb903a760",
                Kind::JarRawV1 => "e099b93f55bf83faab7149172198f914eabc25e42535bbc627ad0f4f2833a3c5",
                Kind::RawSha256 => "97feff80494a54f1b5001f6f4bbdbd45cb64ccbb2dffeb679da9da9be0434b07",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/guava-19.0.jar",
            combined! {
                Kind::JarRawV1 => "c44bee0c1835c70c899b3e67b16d1eb2961e291e614d6213bdfbb283e1332937",
                Kind::RawSha256 => "58d4cc2e05ebb012bbac568b032f75623be1cb6fb096f3c60c72a86f7f057de4",
                Kind::JarClassV1 => "95aee2084dbaf92ffcfa933d62f8a69a1af9a0db979356798ffe650f91d8414f",
                Kind::JarMavenCentralV1 => "6ce200f6b23222af3d8abb6b6459e6c44f4bb0e9",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/httpclient-cache-4.5.14.jar",
            combined! {
                Kind::JarClassV1 => "8fa457b332a78ec94d87f88404fff79090c100ddf2ffd564efc03d7dbe7789a5",
                Kind::JarMavenCentralV1 => "17e68860da010d0002c2cd05349d6013ef67ab64",
                Kind::JarRawV1 => "ffde1edd6702e96aebfe585087814ae660a8db31ec63f533451c5df6ff04c4e6",
                Kind::RawSha256 => "5324d2cbc2d311c9f91b82bcbc746ec2a29f1f5b744395a50ff3afb873db1cee",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/java-support-7.5.1.jar",
            combined! {
                Kind::RawSha256 => "391d97def8b84474176f4144012be7a67ed3f77ab4111ef41e4ed232f9e07b64",
                Kind::JarRawV1 => "ae8aaf32556f051325cd97ec33e4a01ae46df0b44b17cca32ed706b4786a755b",
                Kind::JarClassV1 => "cfd64420bfb541613084ebe96e501c125abde2c875fee8cc7c389705a6383101",
                Kind::JarMavenCentralV1 => "c3fecaa141e8f0fff8a14e6800aefa8155c9b3e8",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/log4j-slf4j-impl-2.17.1.jar",
            combined! {
                Kind::JarClassV1 => "9ad437ba29a9f5f245d4ccd6fc067aaa9a4e02dc7c677d61ffbc395738971862",
                Kind::JarMavenCentralV1 => "84692d456bcce689355d33d68167875e486954dd",
                Kind::JarRawV1 => "a78204ec2fbf25286ad7f7c7d497843d9758c49b55895c8a413998b74a10428e",
                Kind::RawSha256 => "e9a03720e5d5076009c2530635da9d08485e28a0b0ec20708dadc51afb78e41e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/metrics-core-3.2.2.jar",
            combined! {
                Kind::JarClassV1 => "128a2a3a64c5bb2ac06d949b38774d0b6db58aeb330caaed07fb50b28711be78",
                Kind::JarMavenCentralV1 => "cd9886f498ee2ab2d994f0c779e5553b2c450416",
                Kind::RawSha256 => "5c6f685e41664d10c70c65837cba9e58d39ff3896811e3b5707a934b11c85ad0",
                Kind::JarRawV1 => "e0bd824dcf66c22830cd59080b1ee7e7054046fc4fd47593193a90eca60cace9",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-core-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "30206cc29f4da57fdd128cfd01c1ac896ec11ae3ca1931928f643a32abcaebc0",
                Kind::JarMavenCentralV1 => "0958fae127de9e8b0296e6f089c7451b6d5f0846",
                Kind::RawSha256 => "da8c91082b618bd02bd6300c688b9336926ba49b6206b3fefb421dccee288182",
                Kind::JarRawV1 => "648d7df7af428052309e2a7b637e98ec9ae1f7db55f4cb4670cef0081d17e297",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-messaging-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "b9b561c29bac7d7103458e87323b079696dae6f098307a6342f6cbaf6ed10395",
                Kind::JarMavenCentralV1 => "e3ec93dfbf90c451e9f7fb34a3e33a6ac60edd31",
                Kind::RawSha256 => "4daec9276f140b72d79e9ded314c07077bfb51d67d13207832edd282ef0d95eb",
                Kind::JarRawV1 => "545763f27b6878cbffb60b3216df562315d99b1d28fad2dfe14496046051a0e9",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-messaging-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "7cd6075a4590120d7863a83d6619c3f094776c6ebe542a33d48ca4901d70cb50",
                Kind::JarClassV1 => "d275f9b50750aef585e3d0ce97513a142e4b2911ecafa44d977781de99bd1cf1",
                Kind::JarMavenCentralV1 => "beaca9bd69ad861dbb55f1694853a02cb6988ae7",
                Kind::JarRawV1 => "c531a483b3e1fb4654e63a213a5fb07dacec89b0b84ae6a71d112ecb04808f32",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-profile-api-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "e04c83aa6115da14878c642d008586ff11c0263ad723a3153f7153806da9c5a0",
                Kind::JarRawV1 => "bdb3ba259b74280ee850e3e4f9d123190517a1a839e3ae55ef150a55c6313546",
                Kind::JarClassV1 => "6b0ed5a5fda57c040492cb9d634a361e9d6439ff6aaedce52939e5f69be6ed20",
                Kind::JarMavenCentralV1 => "bb0a1f97d38342a5715bad628ee24000b08e821e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-profile-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "bc67c64652f060d86b64649ca0345dd0c9e49cc622bd6ed7637e41845cd2db33",
                Kind::JarRawV1 => "578bd06971997cea753a407394d6a63166e7219df5e251afd341e7b0abdd2dd4",
                Kind::JarClassV1 => "ba5131ef4582ab0810e11ea7c1cad42e5bf903a533aecd19f795242951293b62",
                Kind::JarMavenCentralV1 => "6cb4595c7a988d964f6a2d55dcac754b0c68904e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-saml-api-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "5a1f3ab432f5163df8fbf7d0083bab9801a183d7544ac60b7f4bbccc7f59bc18",
                Kind::JarClassV1 => "86f859d1e74adbba0e36981834b019e6fb516133b846e0a315b8cec032b83243",
                Kind::JarMavenCentralV1 => "bef43d21b2d878baceae291af4a0ad3449c7d7ec",
                Kind::JarRawV1 => "34430593937be9c9fc61cda99a8215ced2020ed044d15c9dccf0ef99f598d5ce",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-saml-impl-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "540234ba1c0ae9b4150a5a06bbcf0108a4797a5704801c2db937b058b04f1ab9",
                Kind::JarClassV1 => "4fb31d9dbfdc8a72d36988302a64c8310806177083978c87fa98d6e76756e8da",
                Kind::JarMavenCentralV1 => "ecf4a9552575d38cffd4dc56d95e7564b7dccfc1",
                Kind::RawSha256 => "4e53cdbb3e95664535ed0070164ee074ea164336ce6a31beb3d1356385253051",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-security-api-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "8c52fab5aa69ca0be3dad921d2ed8631252de45981589ba363cc6430e3cb214a",
                Kind::JarMavenCentralV1 => "15cbb232ae6665edc5df5f260e551e69fdb362e5",
                Kind::RawSha256 => "a764325cb1ed4d1c137202a07312ebe5cffc161012c7abd721a3a0e4470c3e96",
                Kind::JarRawV1 => "d625acaff8578c41b676baa4d203af41b633331cd7d275bbb05f17ff660f0a56",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-security-impl-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "35b28d8b5be147355c012370bd55295a268d29afc7c91454064b655fc21867f2",
                Kind::JarClassV1 => "1de11e29cb2cc56cd314743446e72346dcc2f0142cec73242b9eb588e01fe747",
                Kind::JarMavenCentralV1 => "b2bc1aa5b0f400aa50499f3783b10e9f7c216a47",
                Kind::RawSha256 => "e18bdf84f021b3737c53e2d27bb4853e944abb3365a792f710da1571df9832a0",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-soap-api-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "841427d8eecec98287f4fdc5fc9acc706db16692fddc1a2adbefc6fd50f7f627",
                Kind::JarRawV1 => "877ddbbbd5f832578308591e3835fe771cc75775d0c5cf8da8edba6ffd507c6e",
                Kind::JarClassV1 => "313dcb824a955877cc29b8a0045c4aec66b33192990c3e16591dc0805d760747",
                Kind::JarMavenCentralV1 => "c497df002980c6e482ce7b828924bb24f60f99f7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-soap-impl-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "74db32205f0917798c4bf629dc181282ab8de9086440037185d6b0fea419041d",
                Kind::JarMavenCentralV1 => "30ed8d37259e840df5b3fd8daf7b654129a9190c",
                Kind::JarRawV1 => "19190db7c20a5a7a5addad86e0c2097d3eda8b1bc651b00646a63e52ab6ccdf2",
                Kind::RawSha256 => "9e499b084fff148b8c068a16d40035a3ba76283eb2250a37fca45eabbc3cb2e4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-storage-api-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "5d64e2aa94e440413f063124e1bd5ea14008fa0611854c9ef508eadc57871998",
                Kind::JarRawV1 => "6d76e7f3725086e63a4f944eb369e5e64b32600e1c6a90051467317939f14ff0",
                Kind::JarClassV1 => "edec8d01835f81c4346420b7bf04a0590f856f3bd5cb987ea4a81d6ffd9ff9d5",
                Kind::JarMavenCentralV1 => "a984671fd04e50da03f68003d2b062578e63ec86",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-storage-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "d7cfbce5fb8e7b6c0fa9c690cb0ec801cfe63d51798795452df3177d5838cefc",
                Kind::JarRawV1 => "20bccf256554e0a6ddf153559a275319b00fb5088f9115c1ef3ad85324b7537e",
                Kind::JarClassV1 => "b7259ee454e65a560d3458d429b6dd815cebea01258e7208d81d64acae1538e0",
                Kind::JarMavenCentralV1 => "a4b828fe1a9d64953ecdd8a9e00ff31b63ad6ef0",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-xmlsec-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "a42ca8103feba47d644962e1102c07753fcf8f4f38191afb7e06d73c9a3d1c91",
                Kind::RawSha256 => "fa1b5971556e26b66a4a6dc119d17b6c1f99b3d3288ae36dfbaaf262330c03da",
                Kind::JarClassV1 => "52d22e95423f03400cf40b5c06cfbda7840eefa0d5d3c4d8358b5dd64e926bae",
                Kind::JarMavenCentralV1 => "a1b10f97deca1e3405f95db5b39697c0d46f5e0d",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/opensaml-xmlsec-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "6bb4be41b1aa9e9f6a122504b77cc85a16f11427fb9ecd6d096bbaf0e5a882df",
                Kind::JarRawV1 => "07470be684e0a25d384c89523ad9fca603d54c5927a335708303e42a905edc22",
                Kind::JarClassV1 => "758e9b563673cb2fffe760102fe5f2a6d735810fe79cbaf2ef1bcfdf5f9298c8",
                Kind::JarMavenCentralV1 => "d46cb9854a1ff85bea34ece7077bc32dbc2f10da",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/slf4j-api-1.6.2.jar",
            combined! {
                Kind::JarRawV1 => "51bf440ef88e6f86d2bb6097c537b5afaf8ca93a7fe4a3ee1a203113a6f279d3",
                Kind::JarClassV1 => "af24a6f8c98dd2a81bfbfa9503452c31b64900b76e33a258e102086f661c1133",
                Kind::JarMavenCentralV1 => "8619e95939167fb37245b5670135e4feb0ec7d50",
                Kind::RawSha256 => "c26413df1741466d8bec549a4b7f6480acb2ac928b156be8c5e3feaf9cba1b59",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/x-pack-identity-provider-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "eece9bb28f366440830bfa484b151407375b3d5a6a30a78ad03bb74d4a9f9d53",
                Kind::JarMavenCentralV1 => "948185306ed3ef291fd672593c4c4d1c566f2651",
                Kind::RawSha256 => "5befd25f19af2bd2b6a6fdbf516bd560c42e48c03f4f6e287d2663d6a29a84cc",
                Kind::JarRawV1 => "45e7c00c57cf33163d0c819461080fb903ac7fbc1abe6614f897c97771d759d1",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-identity-provider/xmlsec-2.1.4.jar",
            combined! {
                Kind::JarRawV1 => "f3d89c5f5cbbd574c6e0553e412760d6d0c0a9847e3a79b0c4d4df2b040f6e5c",
                Kind::JarClassV1 => "16fcca024d61653302b7ae4a4a3997eae7e93a7c6a34a8f6fe8bed60de02cff0",
                Kind::JarMavenCentralV1 => "cb43326f02e3e77526c24269c8b5d3cc3f7f6653",
                Kind::RawSha256 => "2e2ec8fe0cf873979f630ae4d35e7ede3390321279b7a15de9deed3f3430990c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ilm/x-pack-ilm-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "3c16485cba2c4e479bdb070d309f1c207ba952f6c7363cb850b4fcb2d868714e",
                Kind::RawSha256 => "b14481e96a8d0e2e3d9adb94cbd90d8e2f3e14256581b9b24bc94e06e616da56",
                Kind::JarClassV1 => "8c5ee3d31ff411c76ea7a9cfe87368e87edd2ef0a76b3710c1609dbcca33f3a4",
                Kind::JarMavenCentralV1 => "622dc5a1b6a8d9ac84aef7e9b5aefa18c40c8ada",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-logstash/x-pack-logstash-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "02e0fcf1707016cbf78a8c7b7a6e56750abb20d23f8b8dc86bd4a4af9a5596a6",
                Kind::JarMavenCentralV1 => "26c16567cba6427ec44dc5123c719e65a9316f78",
                Kind::RawSha256 => "b80481f047fc81bcee36ad35f080355f695abe0e04d347e3bd561b7e91516c61",
                Kind::JarRawV1 => "374883e806f1ef106f8b8d559dfeb4ed3ac41a06eb7a0e8763d58b4fd56e760c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ml/commons-math3-3.6.1.jar",
            combined! {
                Kind::JarClassV1 => "6bf92ce1f74505c615e5224765586998b0f9aecfa2d790479cd668f28d7101dd",
                Kind::JarMavenCentralV1 => "e4ba98f1d4b3c80ec46392f25e094a6a2e58fcbf",
                Kind::JarRawV1 => "de8a86e786ccdfae373551eee29b07428acdc7ac9eeb495f5b23fed90f36ff35",
                Kind::RawSha256 => "1e56d7b058d28b65abd256b8458e3885b674c1d588fa43cd7d1cbb9c7ef2b308",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ml/elasticsearch-grok-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "e6b6ef74174dec6b1c50d3cb6bf82d73413e56d9d038ddc7e06a26e1eb6fea14",
                Kind::RawSha256 => "a2267aef94e0a815521d5cb233aacb512719cd96721f64f056dbc085397f761e",
                Kind::JarClassV1 => "fee1097201de7908879f65cdaf0fc54bc60a4e023450ca1f9eaf3ffb7b14bc84",
                Kind::JarMavenCentralV1 => "69afea1244752b8cd4b212e3c942e4a1ec3191c6",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ml/jcodings-1.0.44.jar",
            combined! {
                Kind::JarClassV1 => "4dd715e0832222e95234af07d22995a0cc40fdc7328ec88b355877d060650532",
                Kind::JarMavenCentralV1 => "a6884b2fd8fd9a56874db05afaa22435043a2e3e",
                Kind::RawSha256 => "49190d6ad09056de57d7ed41ed5b4b105e033557b5dd170702decdcf05ee341a",
                Kind::JarRawV1 => "3ff823499e3d4fd4815efbf4a54311290f5a64592d944e901f8a2940009b35dd",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ml/joni-2.1.29.jar",
            combined! {
                Kind::JarRawV1 => "5f5c1894e9780c6e408de3baeee7585886d8fbc3ae73b6af25722912567cdcff",
                Kind::JarClassV1 => "e9629b51e39e64cd10c5a7a7e10dbb0c5428ce4006a864da5fba4e1a1ec03ff2",
                Kind::JarMavenCentralV1 => "3cb751702e1194ff24259155db4d37e9383d4320",
                Kind::RawSha256 => "aa4b71415682f3d7fa44083cd94a9ec48478ec3b9c30947b4152913d41b1004d",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ml/super-csv-2.4.0.jar",
            combined! {
                Kind::RawSha256 => "cb3cc48f3cb521a6eb90b2984f98935dce4f184d43ff4aba052f4749a4131d4c",
                Kind::JarClassV1 => "1d57e9b8ab563b3c964935a3cee6aa96a4f3d4d83adddeb98a5076b6a10aee86",
                Kind::JarMavenCentralV1 => "017f8708c929029dde48bc298deaf3c7ae2452d3",
                Kind::JarRawV1 => "f7d9ca7b26a2e4495ac6f4b864d0749d2ab844c769ebc262149c37fd8cdb0287",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ml/x-pack-ml-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "1f02281d123f5050f74ca06c2c6b09952d80e347242da3bb2e473748c978b228",
                Kind::JarClassV1 => "87a79ab7342ecee5863e149468ad1918a381af9dc161ac72933e2f2bc1be364e",
                Kind::JarMavenCentralV1 => "1c31235077fd5d877dafdf8d63ab239d168c2f37",
                Kind::RawSha256 => "ab23499eee94f1dd965bf76587c666d8b2108cd775ae6212646ee396ee20394c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-monitoring/elasticsearch-rest-client-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "91cfab57383a284c826a6dd513dcfba309aea0b473af18582552c18c8b5dd078",
                Kind::JarRawV1 => "08b04d8bc63a2cc976bdad6b6b043d3f17e3c0437450ccea6be2c7316d806c8d",
                Kind::JarClassV1 => "0c09988e2c90bf3b57c1ace621095fdaf1232fed467a8306c8228975a313a77b",
                Kind::JarMavenCentralV1 => "810c3578e58f02e112a918f589b153222891d41b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-monitoring/elasticsearch-rest-client-sniffer-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "a5a81a9ffd9a575102053cba386bac255c70e5ab03a94ae1173526512b658c46",
                Kind::RawSha256 => "6285c145746f0a9d0f7018fb9624fe518562cc5f6cebdf7030f6ad390c3d823c",
                Kind::JarClassV1 => "a1dd5002b52f0cc9b2f37176ce492036514df0e26fc693b0572a549d53969875",
                Kind::JarMavenCentralV1 => "e1731fdf0273e46d5f165f7b3c1db4c398d2b141",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-monitoring/x-pack-monitoring-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "d191f4f8e082bebbbed6743d4edc5c6ee1db569bbc838174bf5c9296ed68f5ea",
                Kind::JarClassV1 => "b2eadff8f3958fb1020f8122f0754aa0f03b4326a3ba130a357223aa864c3d2e",
                Kind::JarMavenCentralV1 => "c87801621b55e004a701bbbd07f3c8b3f391f09b",
                Kind::JarRawV1 => "ee5bf2c4c90beb2c806dfc7a061446115eaf485a7149e5c35e01f08670dc92d4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ql/antlr4-runtime-4.9.2.jar",
            combined! {
                Kind::RawSha256 => "120053628dd598d43cb7ac6b9ecc72529dfa5a5fd3292d37cf638a81cc0075f6",
                Kind::JarClassV1 => "06b44f4b2db1cb279833f864d2f955d9015337e2278406269cf5070fc30aaca3",
                Kind::JarMavenCentralV1 => "ece33ec76e002dfde574cf7b57451a91a99185c5",
                Kind::JarRawV1 => "751394f157b9a82d14cf82bdc1d9c84406ae49c51baa7cc3b2e0b6c4eaef36bc",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-ql/x-pack-ql-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "cc0d937d568bd98c0cffb80bd085d691ff9a3e27b4234f5541bf0789b76f600b",
                Kind::JarClassV1 => "544217468bae50f757e557156ef8fc06c192833af33965dd2d27fa5c3b8cf569",
                Kind::JarMavenCentralV1 => "e353dfbffb1250312c39ef7072bc88941493f991",
                Kind::RawSha256 => "e2403ec885d351523ed44347132f6162815ca84ed4cc217317d07155a0730155",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-rollup/x-pack-rollup-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "c42df93fbb23310c34e90ab815ee79e59331b5551c4acb75b3d89ae18c6cacd9",
                Kind::JarMavenCentralV1 => "164c9e3ced65d4b0962d59920b33bb0ffa9e2287",
                Kind::RawSha256 => "028a10d21900bc052fba41515796e0d76430d5ef3aa70fc870d52d5212b9c2ad",
                Kind::JarRawV1 => "4fb174d95e40327e33cc30fc3a0cfd96dd1d63a4ab83d42d77c849dc8be0724b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/accessors-smart-2.4.2.jar",
            combined! {
                Kind::JarClassV1 => "70e28536d92a7e3063561f9eec510155d1051e450a264890571ffc5b1ba3c14d",
                Kind::JarMavenCentralV1 => "4f09981a3c80f0766998c68d83bfd060812d5bcd",
                Kind::JarRawV1 => "6c5a85ffdbdfd1cbe3ebf9112f3aa6d5af4e2b58b040743d1ea4b08a6df98d8e",
                Kind::RawSha256 => "0972bbc99437c4163acd09b630e6c77eab4cfab8a9594621c95466c0c6645396",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/asm-9.1.jar",
            combined! {
                Kind::JarRawV1 => "d7e561bcafa78354f3934bbb4365f6f84236f02e2c0e2d9291ce8b2ffaf57f2f",
                Kind::JarClassV1 => "946cd8d0c6cef02fe3b4f475ea5acc52d9c761a702baa53f6b10ae4476c9d296",
                Kind::JarMavenCentralV1 => "a99500cf6eea30535eeac6be73899d048f8d12a8",
                Kind::RawSha256 => "cda4de455fab48ff0bcb7c48b4639447d4de859a7afc30a094a986f0936beba2",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/cryptacular-1.2.4.jar",
            combined! {
                Kind::JarClassV1 => "cc49125e2781a6a2e6fa1b2202d2f3ec7879ec0df717421e4108dd24b5d97309",
                Kind::JarMavenCentralV1 => "4994c015d87886212683245d13e87f6fb903a760",
                Kind::RawSha256 => "97feff80494a54f1b5001f6f4bbdbd45cb64ccbb2dffeb679da9da9be0434b07",
                Kind::JarRawV1 => "e099b93f55bf83faab7149172198f914eabc25e42535bbc627ad0f4f2833a3c5",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/guava-19.0.jar",
            combined! {
                Kind::JarClassV1 => "95aee2084dbaf92ffcfa933d62f8a69a1af9a0db979356798ffe650f91d8414f",
                Kind::JarMavenCentralV1 => "6ce200f6b23222af3d8abb6b6459e6c44f4bb0e9",
                Kind::RawSha256 => "58d4cc2e05ebb012bbac568b032f75623be1cb6fb096f3c60c72a86f7f057de4",
                Kind::JarRawV1 => "c44bee0c1835c70c899b3e67b16d1eb2961e291e614d6213bdfbb283e1332937",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/httpclient-cache-4.5.14.jar",
            combined! {
                Kind::JarRawV1 => "ffde1edd6702e96aebfe585087814ae660a8db31ec63f533451c5df6ff04c4e6",
                Kind::RawSha256 => "5324d2cbc2d311c9f91b82bcbc746ec2a29f1f5b744395a50ff3afb873db1cee",
                Kind::JarClassV1 => "8fa457b332a78ec94d87f88404fff79090c100ddf2ffd564efc03d7dbe7789a5",
                Kind::JarMavenCentralV1 => "17e68860da010d0002c2cd05349d6013ef67ab64",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/jakarta.mail-1.6.3.jar",
            combined! {
                Kind::JarRawV1 => "2167df3dfa9546ff745eae83c51e1cb25cd9c33c490e80a496a9582d829d5196",
                Kind::RawSha256 => "018ffd5684fd758157886933cc74116996d7f5757cc6f104bb43a647a3815f8a",
                Kind::JarClassV1 => "bbad1626bc0a6b85c0900559032ce300b47c6dea0cf10d0f0e0eb0e73419f5c3",
                Kind::JarMavenCentralV1 => "787e007e377223bba85a33599d3da416c135f99b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/java-support-7.5.1.jar",
            combined! {
                Kind::JarRawV1 => "ae8aaf32556f051325cd97ec33e4a01ae46df0b44b17cca32ed706b4786a755b",
                Kind::JarClassV1 => "cfd64420bfb541613084ebe96e501c125abde2c875fee8cc7c389705a6383101",
                Kind::JarMavenCentralV1 => "c3fecaa141e8f0fff8a14e6800aefa8155c9b3e8",
                Kind::RawSha256 => "391d97def8b84474176f4144012be7a67ed3f77ab4111ef41e4ed232f9e07b64",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/jcip-annotations-1.0.jar",
            combined! {
                Kind::RawSha256 => "be5805392060c71474bf6c9a67a099471274d30b83eef84bfc4e0889a4f1dcc0",
                Kind::JarClassV1 => "0fb33f79073ee3e1c21fd5007e20f8834b702cfcd01004e478609b35d4a122e2",
                Kind::JarMavenCentralV1 => "afba4942caaeaf46aab0b976afd57cc7c181467e",
                Kind::JarRawV1 => "c93da171e66febbd2b43baeb1d705b15055e71fc76b23b86f6713e2d613c668a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/json-smart-2.4.10.jar",
            combined! {
                Kind::JarRawV1 => "eb6ff90c2bb1872387e04577ee3b96679e2a073f9df90dce1cb024e4fe8d2f9c",
                Kind::JarClassV1 => "087006cd13aba169825b563038e964f980dd44b2033fb27cdd7a53f540c88fdf",
                Kind::JarMavenCentralV1 => "91cb329e9424bf32131eeb1ce2d17bf31b9899bc",
                Kind::RawSha256 => "70cab5e9488630dc631b1fc6e7fa550d95cddd19ba14db39ceca7cabfbd4e5ae",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/lang-tag-1.4.4.jar",
            combined! {
                Kind::JarClassV1 => "204546610f8e9e399c584e89bd20834fa03bc6d86046325919aeee15e3c98fc9",
                Kind::JarMavenCentralV1 => "1db9a709239ae473a69b5424c7e78d0b7108229d",
                Kind::RawSha256 => "e49d2c694bb80c7036c177f2aabf53b7156061a68bd19dfd60e2bd370709e0c5",
                Kind::JarRawV1 => "71897cfddeecc474bb9ffb52555edd5372272ece6bbc153d9b49e512d1985dc9",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/log4j-slf4j-impl-2.17.1.jar",
            combined! {
                Kind::JarClassV1 => "9ad437ba29a9f5f245d4ccd6fc067aaa9a4e02dc7c677d61ffbc395738971862",
                Kind::JarMavenCentralV1 => "84692d456bcce689355d33d68167875e486954dd",
                Kind::RawSha256 => "e9a03720e5d5076009c2530635da9d08485e28a0b0ec20708dadc51afb78e41e",
                Kind::JarRawV1 => "a78204ec2fbf25286ad7f7c7d497843d9758c49b55895c8a413998b74a10428e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/metrics-core-3.2.2.jar",
            combined! {
                Kind::JarClassV1 => "128a2a3a64c5bb2ac06d949b38774d0b6db58aeb330caaed07fb50b28711be78",
                Kind::JarMavenCentralV1 => "cd9886f498ee2ab2d994f0c779e5553b2c450416",
                Kind::JarRawV1 => "e0bd824dcf66c22830cd59080b1ee7e7054046fc4fd47593193a90eca60cace9",
                Kind::RawSha256 => "5c6f685e41664d10c70c65837cba9e58d39ff3896811e3b5707a934b11c85ad0",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/nimbus-jose-jwt-9.23.jar",
            combined! {
                Kind::JarRawV1 => "199f48a802f62d78028f038765bd4b684a6ca5af347e979f3627c65f73dd81da",
                Kind::JarClassV1 => "761cc58ab0a8f3e2c97fee7f0fd1c59e95526ca5a0705aa8bf7e6de841c81764",
                Kind::JarMavenCentralV1 => "e2fb2b3784fffdba251db0c29c9f7cb42c670906",
                Kind::RawSha256 => "33ab8084fdae1d75be1b061b1489d4a12045bd7b50c2e24ff152911e4551ec07",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/oauth2-oidc-sdk-9.37.jar",
            combined! {
                Kind::JarClassV1 => "c967ede438d8736f6b089491f136b5ab1dc51ad109f87b312fb8cfdbb05c87ff",
                Kind::JarMavenCentralV1 => "d0d7af8ef989212fd9cac52a6ecfac907a515b5c",
                Kind::RawSha256 => "44a04bbed5ae3f6d198aa73ee6b545c476e528ec1a267ef3e9f7033f886dd6fe",
                Kind::JarRawV1 => "c6ed345dc8cabfccb2475cb5439e8b158977a2c8d218eabceabce5056d7bb742",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-core-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "da8c91082b618bd02bd6300c688b9336926ba49b6206b3fefb421dccee288182",
                Kind::JarRawV1 => "648d7df7af428052309e2a7b637e98ec9ae1f7db55f4cb4670cef0081d17e297",
                Kind::JarClassV1 => "30206cc29f4da57fdd128cfd01c1ac896ec11ae3ca1931928f643a32abcaebc0",
                Kind::JarMavenCentralV1 => "0958fae127de9e8b0296e6f089c7451b6d5f0846",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-messaging-api-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "4daec9276f140b72d79e9ded314c07077bfb51d67d13207832edd282ef0d95eb",
                Kind::JarClassV1 => "b9b561c29bac7d7103458e87323b079696dae6f098307a6342f6cbaf6ed10395",
                Kind::JarMavenCentralV1 => "e3ec93dfbf90c451e9f7fb34a3e33a6ac60edd31",
                Kind::JarRawV1 => "545763f27b6878cbffb60b3216df562315d99b1d28fad2dfe14496046051a0e9",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-messaging-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "7cd6075a4590120d7863a83d6619c3f094776c6ebe542a33d48ca4901d70cb50",
                Kind::JarRawV1 => "c531a483b3e1fb4654e63a213a5fb07dacec89b0b84ae6a71d112ecb04808f32",
                Kind::JarClassV1 => "d275f9b50750aef585e3d0ce97513a142e4b2911ecafa44d977781de99bd1cf1",
                Kind::JarMavenCentralV1 => "beaca9bd69ad861dbb55f1694853a02cb6988ae7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-profile-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "bdb3ba259b74280ee850e3e4f9d123190517a1a839e3ae55ef150a55c6313546",
                Kind::RawSha256 => "e04c83aa6115da14878c642d008586ff11c0263ad723a3153f7153806da9c5a0",
                Kind::JarClassV1 => "6b0ed5a5fda57c040492cb9d634a361e9d6439ff6aaedce52939e5f69be6ed20",
                Kind::JarMavenCentralV1 => "bb0a1f97d38342a5715bad628ee24000b08e821e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-profile-impl-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "ba5131ef4582ab0810e11ea7c1cad42e5bf903a533aecd19f795242951293b62",
                Kind::JarMavenCentralV1 => "6cb4595c7a988d964f6a2d55dcac754b0c68904e",
                Kind::JarRawV1 => "578bd06971997cea753a407394d6a63166e7219df5e251afd341e7b0abdd2dd4",
                Kind::RawSha256 => "bc67c64652f060d86b64649ca0345dd0c9e49cc622bd6ed7637e41845cd2db33",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-saml-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "34430593937be9c9fc61cda99a8215ced2020ed044d15c9dccf0ef99f598d5ce",
                Kind::JarClassV1 => "86f859d1e74adbba0e36981834b019e6fb516133b846e0a315b8cec032b83243",
                Kind::JarMavenCentralV1 => "bef43d21b2d878baceae291af4a0ad3449c7d7ec",
                Kind::RawSha256 => "5a1f3ab432f5163df8fbf7d0083bab9801a183d7544ac60b7f4bbccc7f59bc18",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-saml-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "4e53cdbb3e95664535ed0070164ee074ea164336ce6a31beb3d1356385253051",
                Kind::JarClassV1 => "4fb31d9dbfdc8a72d36988302a64c8310806177083978c87fa98d6e76756e8da",
                Kind::JarMavenCentralV1 => "ecf4a9552575d38cffd4dc56d95e7564b7dccfc1",
                Kind::JarRawV1 => "540234ba1c0ae9b4150a5a06bbcf0108a4797a5704801c2db937b058b04f1ab9",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-security-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "d625acaff8578c41b676baa4d203af41b633331cd7d275bbb05f17ff660f0a56",
                Kind::RawSha256 => "a764325cb1ed4d1c137202a07312ebe5cffc161012c7abd721a3a0e4470c3e96",
                Kind::JarClassV1 => "8c52fab5aa69ca0be3dad921d2ed8631252de45981589ba363cc6430e3cb214a",
                Kind::JarMavenCentralV1 => "15cbb232ae6665edc5df5f260e551e69fdb362e5",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-security-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "e18bdf84f021b3737c53e2d27bb4853e944abb3365a792f710da1571df9832a0",
                Kind::JarRawV1 => "35b28d8b5be147355c012370bd55295a268d29afc7c91454064b655fc21867f2",
                Kind::JarClassV1 => "1de11e29cb2cc56cd314743446e72346dcc2f0142cec73242b9eb588e01fe747",
                Kind::JarMavenCentralV1 => "b2bc1aa5b0f400aa50499f3783b10e9f7c216a47",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-soap-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "877ddbbbd5f832578308591e3835fe771cc75775d0c5cf8da8edba6ffd507c6e",
                Kind::RawSha256 => "841427d8eecec98287f4fdc5fc9acc706db16692fddc1a2adbefc6fd50f7f627",
                Kind::JarClassV1 => "313dcb824a955877cc29b8a0045c4aec66b33192990c3e16591dc0805d760747",
                Kind::JarMavenCentralV1 => "c497df002980c6e482ce7b828924bb24f60f99f7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-soap-impl-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "19190db7c20a5a7a5addad86e0c2097d3eda8b1bc651b00646a63e52ab6ccdf2",
                Kind::JarClassV1 => "74db32205f0917798c4bf629dc181282ab8de9086440037185d6b0fea419041d",
                Kind::JarMavenCentralV1 => "30ed8d37259e840df5b3fd8daf7b654129a9190c",
                Kind::RawSha256 => "9e499b084fff148b8c068a16d40035a3ba76283eb2250a37fca45eabbc3cb2e4",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-storage-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "6d76e7f3725086e63a4f944eb369e5e64b32600e1c6a90051467317939f14ff0",
                Kind::JarClassV1 => "edec8d01835f81c4346420b7bf04a0590f856f3bd5cb987ea4a81d6ffd9ff9d5",
                Kind::JarMavenCentralV1 => "a984671fd04e50da03f68003d2b062578e63ec86",
                Kind::RawSha256 => "5d64e2aa94e440413f063124e1bd5ea14008fa0611854c9ef508eadc57871998",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-storage-impl-3.4.5.jar",
            combined! {
                Kind::JarClassV1 => "b7259ee454e65a560d3458d429b6dd815cebea01258e7208d81d64acae1538e0",
                Kind::JarMavenCentralV1 => "a4b828fe1a9d64953ecdd8a9e00ff31b63ad6ef0",
                Kind::RawSha256 => "d7cfbce5fb8e7b6c0fa9c690cb0ec801cfe63d51798795452df3177d5838cefc",
                Kind::JarRawV1 => "20bccf256554e0a6ddf153559a275319b00fb5088f9115c1ef3ad85324b7537e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-xmlsec-api-3.4.5.jar",
            combined! {
                Kind::JarRawV1 => "a42ca8103feba47d644962e1102c07753fcf8f4f38191afb7e06d73c9a3d1c91",
                Kind::RawSha256 => "fa1b5971556e26b66a4a6dc119d17b6c1f99b3d3288ae36dfbaaf262330c03da",
                Kind::JarClassV1 => "52d22e95423f03400cf40b5c06cfbda7840eefa0d5d3c4d8358b5dd64e926bae",
                Kind::JarMavenCentralV1 => "a1b10f97deca1e3405f95db5b39697c0d46f5e0d",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/opensaml-xmlsec-impl-3.4.5.jar",
            combined! {
                Kind::RawSha256 => "6bb4be41b1aa9e9f6a122504b77cc85a16f11427fb9ecd6d096bbaf0e5a882df",
                Kind::JarRawV1 => "07470be684e0a25d384c89523ad9fca603d54c5927a335708303e42a905edc22",
                Kind::JarClassV1 => "758e9b563673cb2fffe760102fe5f2a6d735810fe79cbaf2ef1bcfdf5f9298c8",
                Kind::JarMavenCentralV1 => "d46cb9854a1ff85bea34ece7077bc32dbc2f10da",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/slf4j-api-1.6.2.jar",
            combined! {
                Kind::RawSha256 => "c26413df1741466d8bec549a4b7f6480acb2ac928b156be8c5e3feaf9cba1b59",
                Kind::JarRawV1 => "51bf440ef88e6f86d2bb6097c537b5afaf8ca93a7fe4a3ee1a203113a6f279d3",
                Kind::JarClassV1 => "af24a6f8c98dd2a81bfbfa9503452c31b64900b76e33a258e102086f661c1133",
                Kind::JarMavenCentralV1 => "8619e95939167fb37245b5670135e4feb0ec7d50",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/x-pack-security-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "9123967f868507237f4a3986def5671d52097ca2c9c2f6aaefa0c70a39c8032c",
                Kind::JarMavenCentralV1 => "768e9b40d02d917b68741046295e34e50ba5085e",
                Kind::RawSha256 => "2db68f53cb6da4d382e703640bf989d28232bbf7b3e5849c381d1a7239479fdc",
                Kind::JarRawV1 => "49f1218baa4cbbfc74cf1bfad9ad8235e16b3559280a686ea9535e624ead72c6",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-security/xmlsec-2.1.4.jar",
            combined! {
                Kind::JarClassV1 => "16fcca024d61653302b7ae4a4a3997eae7e93a7c6a34a8f6fe8bed60de02cff0",
                Kind::JarMavenCentralV1 => "cb43326f02e3e77526c24269c8b5d3cc3f7f6653",
                Kind::JarRawV1 => "f3d89c5f5cbbd574c6e0553e412760d6d0c0a9847e3a79b0c4d4df2b040f6e5c",
                Kind::RawSha256 => "2e2ec8fe0cf873979f630ae4d35e7ede3390321279b7a15de9deed3f3430990c",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-shutdown/x-pack-shutdown-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "ea93e2f4e3d48ca5b799295c101de93879b904c1d78c4bab32650de23d2f41a5",
                Kind::RawSha256 => "ffe3ce3837de1e17c4fe67ef01c75972b7825c516914adf34189bc5430aef3ec",
                Kind::JarClassV1 => "ed549b65e570a94d266580e311a80ac950cffbb7c662458becb000d306658efe",
                Kind::JarMavenCentralV1 => "37b098d687db5f80c9d61bf7f2bec084e7589fac",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-sql/aggs-matrix-stats-client-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "be8acdb650fdc844979f14fdd4ec70da12f67e023fa174bd0c6fa10ce3ec294e",
                Kind::RawSha256 => "9bfe710b4762e6bd3fb28dcc07f9ff2aa4f36fdb2aad1dfc5bd6505ad9b4b404",
                Kind::JarClassV1 => "aa2a5bcac58731f1c102036b5030ba2b520728591696dde4dc45c6623127fdea",
                Kind::JarMavenCentralV1 => "a95e2b2438857705492134bf0392cbfb1af748f1",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-sql/sql-action-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "78af2f180170efb9d454e5baa3dadaed9a53d2f575a4bfd1bf16851969585f00",
                Kind::JarRawV1 => "3c2aaabd80b85605622ea9a0fbcdcaccbe847e7e1fcc45837f1977d1c1e110e2",
                Kind::JarClassV1 => "7a8488a0813c0bdba58d548aca86ca409950136ef9cd980154aa4883bd1bd30b",
                Kind::JarMavenCentralV1 => "bef5656643bea85be95079a163b2f07e17ad9d46",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-sql/sql-proto-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "6151486c2370ce74eaea561a52ad9c36f91b9be0ac1fe3804fb2a529c6c47e3c",
                Kind::JarRawV1 => "76303f61405fc3b1f717150350b1617c96c41b5e58fb111abc8123d924586d50",
                Kind::JarClassV1 => "9ef631ca6ad292b6041189ea822b1ffcbc706ae6bd1e0425276f6a30dc90d88b",
                Kind::JarMavenCentralV1 => "e491e9f1d775ae87f11ebdc99052b0735e3e5f05",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-sql/x-pack-sql-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "23cbe12cecc63ff0a586e5ae906419cf8dd336d39b16ab089b23a4ec0f51ca7e",
                Kind::RawSha256 => "dba9616c525211e115176991d18e276ef711881229540fa40c66bdeb622340ae",
                Kind::JarClassV1 => "fcda9fafedc9d0a8a2715da4ffd02350090c92df8ac546178959801d399ed16c",
                Kind::JarMavenCentralV1 => "9ba97ab81de975642f4a300476dd4a41126a58e3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-stack/x-pack-stack-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "6404b9954751127d80267951a9e76308a1c216b33b270d71139711168c51e5c2",
                Kind::JarClassV1 => "2f2d33886dd94ab87918d7d03b8acb3e5683bd8c3b4346c0a825a092cb5de014",
                Kind::JarMavenCentralV1 => "b87d4883d5057c68b94ae2597fcdbbab623a6604",
                Kind::RawSha256 => "a91ec00ad8adc33bcc6701171f8232d68208a216d4e66a20070084b8dd0fe01b",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-text-structure/elasticsearch-grok-7.17.17.jar",
            combined! {
                Kind::RawSha256 => "a2267aef94e0a815521d5cb233aacb512719cd96721f64f056dbc085397f761e",
                Kind::JarRawV1 => "e6b6ef74174dec6b1c50d3cb6bf82d73413e56d9d038ddc7e06a26e1eb6fea14",
                Kind::JarClassV1 => "fee1097201de7908879f65cdaf0fc54bc60a4e023450ca1f9eaf3ffb7b14bc84",
                Kind::JarMavenCentralV1 => "69afea1244752b8cd4b212e3c942e4a1ec3191c6",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-text-structure/icu4j-62.1.jar",
            combined! {
                Kind::JarClassV1 => "ed02dfc3dd7375901fdb24aa1fbdf17da49eed182f38d34bf5c8eccc1c53e16e",
                Kind::JarMavenCentralV1 => "7a4d00d5ec5febd252a6182e8b6e87a0a9821f81",
                Kind::JarRawV1 => "280e4a3a79a5ee084ba59e4fdbe4c59582ea2e56180a5ead031a56cd34f04e36",
                Kind::RawSha256 => "e5b0962e566003d68bf3ec7d87b60e279f5cc7aa93cbc4056432803046512478",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-text-structure/jcodings-1.0.44.jar",
            combined! {
                Kind::RawSha256 => "49190d6ad09056de57d7ed41ed5b4b105e033557b5dd170702decdcf05ee341a",
                Kind::JarRawV1 => "3ff823499e3d4fd4815efbf4a54311290f5a64592d944e901f8a2940009b35dd",
                Kind::JarClassV1 => "4dd715e0832222e95234af07d22995a0cc40fdc7328ec88b355877d060650532",
                Kind::JarMavenCentralV1 => "a6884b2fd8fd9a56874db05afaa22435043a2e3e",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-text-structure/joni-2.1.29.jar",
            combined! {
                Kind::RawSha256 => "aa4b71415682f3d7fa44083cd94a9ec48478ec3b9c30947b4152913d41b1004d",
                Kind::JarClassV1 => "e9629b51e39e64cd10c5a7a7e10dbb0c5428ce4006a864da5fba4e1a1ec03ff2",
                Kind::JarMavenCentralV1 => "3cb751702e1194ff24259155db4d37e9383d4320",
                Kind::JarRawV1 => "5f5c1894e9780c6e408de3baeee7585886d8fbc3ae73b6af25722912567cdcff",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-text-structure/super-csv-2.4.0.jar",
            combined! {
                Kind::RawSha256 => "cb3cc48f3cb521a6eb90b2984f98935dce4f184d43ff4aba052f4749a4131d4c",
                Kind::JarRawV1 => "f7d9ca7b26a2e4495ac6f4b864d0749d2ab844c769ebc262149c37fd8cdb0287",
                Kind::JarClassV1 => "1d57e9b8ab563b3c964935a3cee6aa96a4f3d4d83adddeb98a5076b6a10aee86",
                Kind::JarMavenCentralV1 => "017f8708c929029dde48bc298deaf3c7ae2452d3",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-text-structure/x-pack-text-structure-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "ff7041896bc9d1516df38e07a66bc63b57e643cd057d5091e84233e071dcd365",
                Kind::JarClassV1 => "5fcaee73c8a7e1633c4beb8aae477a26e982f972d00ae3fa7debec1f920f029d",
                Kind::JarMavenCentralV1 => "67693eb61815a1bb8f8155cf18cd661e65002031",
                Kind::RawSha256 => "10885828870d77b738fb0166397e439b8801051c72f83439b76b7e8ee3ae8ce9",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-voting-only-node/x-pack-voting-only-node-7.17.17.jar",
            combined! {
                Kind::JarClassV1 => "942a64c517ea90df3b87c079e3661ab2250374d12639eba72fc76abd51a7f3c9",
                Kind::JarMavenCentralV1 => "62e5b22617c1b28adf6b40415513c789242ccfff",
                Kind::JarRawV1 => "4300491d3d7565f5cbeea9318b310d0bc1f65e18c4c37ce9b90cdc0185286111",
                Kind::RawSha256 => "ae9932a6c004be972cdc6ca904ddff344a2e23882d7d3eeecd85809360748e42",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-watcher/failureaccess-1.0.1.jar",
            combined! {
                Kind::JarClassV1 => "98aebe7db02141ace8153c2e3dda72bf4657ce4623fbfea2e58e2b16560eb16e",
                Kind::JarMavenCentralV1 => "1dcf1de382a0bf95a3d8b0849546c88bac1292c9",
                Kind::RawSha256 => "a171ee4c734dd2da837e4b16be9df4661afab72a41adaf31eb84dfdaf936ca26",
                Kind::JarRawV1 => "31a953705a18c0d8c4a90955d8b2e21e5ed43f0301e2c54728ff10d0bed33d94",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-watcher/guava-32.0.1-jre.jar",
            combined! {
                Kind::JarRawV1 => "68ad8c8bd943eeb8316e2abb86a0ac0b0d691bb642a94a28e8035cd21becee7a",
                Kind::JarClassV1 => "fba2d6b69c970b7b178be0ac4165504d7045c3fe032539081bc2748e053dec04",
                Kind::JarMavenCentralV1 => "6e5d51a72d142f2d40a57dfb897188b36a95b489",
                Kind::RawSha256 => "bd7fa227591fb8509677d0d1122cf95158f3b8a9f45653f58281d879f6dc48c5",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-watcher/jakarta.activation-1.2.1.jar",
            combined! {
                Kind::RawSha256 => "d84d4ba8b55cdb7fdcbb885e6939386367433f56f5ab8cfdc302a7c3587fa92b",
                Kind::JarRawV1 => "d2bf36ebf70edac4242ed5f0650a0b6ff1a883fa7c5a1abf06be62b1457afca6",
                Kind::JarClassV1 => "94fe3153c5d00d7c5dda0e155d0ea7a24b2766f2a04e3fd7002212f28443f8d2",
                Kind::JarMavenCentralV1 => "8013606426a73d8ba6b568370877251e91a38b89",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-watcher/jakarta.mail-1.6.4.jar",
            combined! {
                Kind::JarRawV1 => "be0291c7ac8dcbbb240c676ee0bf2f616f29d5deeb21b5d8e8df7784e87d5ce3",
                Kind::RawSha256 => "65d4c18e15ea2b9eb129098ae92db4cf996d85179f30ac34f7d4db856ffaa3f9",
                Kind::JarClassV1 => "832c3daba0fc677319db6860894ac2eee66422ec80ed3b56e6dcaa053bff766f",
                Kind::JarMavenCentralV1 => "5015f335c2b974b1a7d08718edc326f0dc613c8a",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-watcher/owasp-java-html-sanitizer-20211018.2.jar",
            combined! {
                Kind::RawSha256 => "48234cd74e35d91a31a683820a35b5b6d11b55527f32a5b162c6757408b95d7a",
                Kind::JarClassV1 => "ee1e9b0f2c6686c51c760e4bb0ad1374686067e4836ab011b2d3abe9826c87ee",
                Kind::JarMavenCentralV1 => "a3226c13cf72633122e94810a53e60529dae2b80",
                Kind::JarRawV1 => "1af502f5a9e80a2a3014556476e3c3c2ad54f88df3a415218b9d2477a31ae5c7",
            },
        ),
        DiscoveredJar::new(
            "usr/share/elasticsearch/modules/x-pack-watcher/x-pack-watcher-7.17.17.jar",
            combined! {
                Kind::JarRawV1 => "d967ee4d0ed330abf1799e369c5dade2d1cd523e83c269dab86a2e171e5e526f",
                Kind::JarClassV1 => "b894eeb57847673ce6a6df9adf85c15858c2b2b230c48967e5f34be0aaf0fa58",
                Kind::JarMavenCentralV1 => "bbffa98f04f912de472df09fd5bc7bcb10b1cffe",
                Kind::RawSha256 => "530bfc82ea5c72e316a0378905d0b2cec757592bbcaa12d6cd829f21fce6be00",
            },
        ),
    ]
}
