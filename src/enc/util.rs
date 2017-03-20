pub fn brotli_max_uint32_t(a: u32, b: u32) -> u32 {
  if a > b { a } else { b }
}


static kLog2Table: [f32; 256] = [0.0000000000000000f32,
                                 0.0000000000000000f32,
                                 1.0000000000000000f32,
                                 1.5849625007211563f32,
                                 2.0000000000000000f32,
                                 2.3219280948873622f32,
                                 2.5849625007211561f32,
                                 2.8073549220576042f32,
                                 3.0000000000000000f32,
                                 3.1699250014423126f32,
                                 3.3219280948873626f32,
                                 3.4594316186372978f32,
                                 3.5849625007211565f32,
                                 3.7004397181410922f32,
                                 3.8073549220576037f32,
                                 3.9068905956085187f32,
                                 4.0000000000000000f32,
                                 4.0874628412503400f32,
                                 4.1699250014423122f32,
                                 4.2479275134435852f32,
                                 4.3219280948873626f32,
                                 4.3923174227787607f32,
                                 4.4594316186372973f32,
                                 4.5235619560570131f32,
                                 4.5849625007211570f32,
                                 4.6438561897747244f32,
                                 4.7004397181410926f32,
                                 4.7548875021634691f32,
                                 4.8073549220576037f32,
                                 4.8579809951275728f32,
                                 4.9068905956085187f32,
                                 4.9541963103868758f32,
                                 5.0000000000000000f32,
                                 5.0443941193584534f32,
                                 5.0874628412503400f32,
                                 5.1292830169449664f32,
                                 5.1699250014423122f32,
                                 5.2094533656289501f32,
                                 5.2479275134435852f32,
                                 5.2854022188622487f32,
                                 5.3219280948873626f32,
                                 5.3575520046180838f32,
                                 5.3923174227787607f32,
                                 5.4262647547020979f32,
                                 5.4594316186372973f32,
                                 5.4918530963296748f32,
                                 5.5235619560570131f32,
                                 5.5545888516776376f32,
                                 5.5849625007211570f32,
                                 5.6147098441152083f32,
                                 5.6438561897747244f32,
                                 5.6724253419714961f32,
                                 5.7004397181410926f32,
                                 5.7279204545631996f32,
                                 5.7548875021634691f32,
                                 5.7813597135246599f32,
                                 5.8073549220576046f32,
                                 5.8328900141647422f32,
                                 5.8579809951275719f32,
                                 5.8826430493618416f32,
                                 5.9068905956085187f32,
                                 5.9307373375628867f32,
                                 5.9541963103868758f32,
                                 5.9772799234999168f32,
                                 6.0000000000000000f32,
                                 6.0223678130284544f32,
                                 6.0443941193584534f32,
                                 6.0660891904577721f32,
                                 6.0874628412503400f32,
                                 6.1085244567781700f32,
                                 6.1292830169449672f32,
                                 6.1497471195046822f32,
                                 6.1699250014423122f32,
                                 6.1898245588800176f32,
                                 6.2094533656289510f32,
                                 6.2288186904958804f32,
                                 6.2479275134435861f32,
                                 6.2667865406949019f32,
                                 6.2854022188622487f32,
                                 6.3037807481771031f32,
                                 6.3219280948873617f32,
                                 6.3398500028846252f32,
                                 6.3575520046180847f32,
                                 6.3750394313469254f32,
                                 6.3923174227787598f32,
                                 6.4093909361377026f32,
                                 6.4262647547020979f32,
                                 6.4429434958487288f32,
                                 6.4594316186372982f32,
                                 6.4757334309663976f32,
                                 6.4918530963296748f32,
                                 6.5077946401986964f32,
                                 6.5235619560570131f32,
                                 6.5391588111080319f32,
                                 6.5545888516776376f32,
                                 6.5698556083309478f32,
                                 6.5849625007211561f32,
                                 6.5999128421871278f32,
                                 6.6147098441152092f32,
                                 6.6293566200796095f32,
                                 6.6438561897747253f32,
                                 6.6582114827517955f32,
                                 6.6724253419714952f32,
                                 6.6865005271832185f32,
                                 6.7004397181410917f32,
                                 6.7142455176661224f32,
                                 6.7279204545631988f32,
                                 6.7414669864011465f32,
                                 6.7548875021634691f32,
                                 6.7681843247769260f32,
                                 6.7813597135246599f32,
                                 6.7944158663501062f32,
                                 6.8073549220576037f32,
                                 6.8201789624151887f32,
                                 6.8328900141647422f32,
                                 6.8454900509443757f32,
                                 6.8579809951275719f32,
                                 6.8703647195834048f32,
                                 6.8826430493618416f32,
                                 6.8948177633079437f32,
                                 6.9068905956085187f32,
                                 6.9188632372745955f32,
                                 6.9307373375628867f32,
                                 6.9425145053392399f32,
                                 6.9541963103868758f32,
                                 6.9657842846620879f32,
                                 6.9772799234999168f32,
                                 6.9886846867721664f32,
                                 7.0000000000000000f32,
                                 7.0112272554232540f32,
                                 7.0223678130284544f32,
                                 7.0334230015374501f32,
                                 7.0443941193584534f32,
                                 7.0552824355011898f32,
                                 7.0660891904577721f32,
                                 7.0768155970508317f32,
                                 7.0874628412503400f32,
                                 7.0980320829605272f32,
                                 7.1085244567781700f32,
                                 7.1189410727235076f32,
                                 7.1292830169449664f32,
                                 7.1395513523987937f32,
                                 7.1497471195046822f32,
                                 7.1598713367783891f32,
                                 7.1699250014423130f32,
                                 7.1799090900149345f32,
                                 7.1898245588800176f32,
                                 7.1996723448363644f32,
                                 7.2094533656289492f32,
                                 7.2191685204621621f32,
                                 7.2288186904958804f32,
                                 7.2384047393250794f32,
                                 7.2479275134435861f32,
                                 7.2573878426926521f32,
                                 7.2667865406949019f32,
                                 7.2761244052742384f32,
                                 7.2854022188622487f32,
                                 7.2946207488916270f32,
                                 7.3037807481771031f32,
                                 7.3128829552843557f32,
                                 7.3219280948873617f32,
                                 7.3309168781146177f32,
                                 7.3398500028846243f32,
                                 7.3487281542310781f32,
                                 7.3575520046180847f32,
                                 7.3663222142458151f32,
                                 7.3750394313469254f32,
                                 7.3837042924740528f32,
                                 7.3923174227787607f32,
                                 7.4008794362821844f32,
                                 7.4093909361377026f32,
                                 7.4178525148858991f32,
                                 7.4262647547020979f32,
                                 7.4346282276367255f32,
                                 7.4429434958487288f32,
                                 7.4512111118323299f32,
                                 7.4594316186372973f32,
                                 7.4676055500829976f32,
                                 7.4757334309663976f32,
                                 7.4838157772642564f32,
                                 7.4918530963296748f32,
                                 7.4998458870832057f32,
                                 7.5077946401986964f32,
                                 7.5156998382840436f32,
                                 7.5235619560570131f32,
                                 7.5313814605163119f32,
                                 7.5391588111080319f32,
                                 7.5468944598876373f32,
                                 7.5545888516776376f32,
                                 7.5622424242210728f32,
                                 7.5698556083309478f32,
                                 7.5774288280357487f32,
                                 7.5849625007211561f32,
                                 7.5924570372680806f32,
                                 7.5999128421871278f32,
                                 7.6073303137496113f32,
                                 7.6147098441152075f32,
                                 7.6220518194563764f32,
                                 7.6293566200796095f32,
                                 7.6366246205436488f32,
                                 7.6438561897747244f32,
                                 7.6510516911789290f32,
                                 7.6582114827517955f32,
                                 7.6653359171851765f32,
                                 7.6724253419714952f32,
                                 7.6794800995054464f32,
                                 7.6865005271832185f32,
                                 7.6934869574993252f32,
                                 7.7004397181410926f32,
                                 7.7073591320808825f32,
                                 7.7142455176661224f32,
                                 7.7210991887071856f32,
                                 7.7279204545631996f32,
                                 7.7347096202258392f32,
                                 7.7414669864011465f32,
                                 7.7481928495894596f32,
                                 7.7548875021634691f32,
                                 7.7615512324444795f32,
                                 7.7681843247769260f32,
                                 7.7747870596011737f32,
                                 7.7813597135246608f32,
                                 7.7879025593914317f32,
                                 7.7944158663501062f32,
                                 7.8008998999203047f32,
                                 7.8073549220576037f32,
                                 7.8137811912170374f32,
                                 7.8201789624151887f32,
                                 7.8265484872909159f32,
                                 7.8328900141647422f32,
                                 7.8392037880969445f32,
                                 7.8454900509443757f32,
                                 7.8517490414160571f32,
                                 7.8579809951275719f32,
                                 7.8641861446542798f32,
                                 7.8703647195834048f32,
                                 7.8765169465650002f32,
                                 7.8826430493618425f32,
                                 7.8887432488982601f32,
                                 7.8948177633079446f32,
                                 7.9008668079807496f32,
                                 7.9068905956085187f32,
                                 7.9128893362299619f32,
                                 7.9188632372745955f32,
                                 7.9248125036057813f32,
                                 7.9307373375628867f32,
                                 7.9366379390025719f32,
                                 7.9425145053392399f32,
                                 7.9483672315846778f32,
                                 7.9541963103868758f32,
                                 7.9600019320680806f32,
                                 7.9657842846620870f32,
                                 7.9715435539507720f32,
                                 7.9772799234999168f32,
                                 7.9829935746943104f32,
                                 7.9886846867721664f32,
                                 7.9943534368588578f32];
pub fn FastLog2(mut v: usize) -> f64 {
  if v < kLog2Table.len() {
    kLog2Table[v] as (f64)
  } else {
    (v as f64).log2()
  }
}
