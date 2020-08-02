
// standard libraries
use std::collections::{HashMap};

// crate utilities
use crate::{
  settings::{
    constants::{
      BIN_SIZE,
      BIN_OVERLAP,
    },
  },
};

// C bindings
use libc::{
  c_int,
  c_double,
};

// R function signature declaration
#[link(name="Rmath")]
extern {
  fn ppois(
    x: c_double,
    lambda: c_double,
    lower_tail: c_int,
    log_p: c_int
  ) -> c_double;
}


fn effective_genome_length_calculator(
  genome_length: f64,
  bin_size: f64,
  bin_overlap: f64,
) -> f64 {
  genome_length * bin_size / bin_overlap
}


fn r_ppoisson(
  lambda: f64,
  psize: usize,
) -> Vec<f64> {
  let mut ppois_vec = vec![0.; psize];
  for ppois_index in 1..=psize {
    // fix lower_tail = TRUE & log_p = FALSE
    unsafe {
      ppois_vec[ppois_index - 1] = 1. - ppois(ppois_index as f64, lambda, 1, 0);
    }
  }
  return ppois_vec
}

fn tabler(
  bined_hm: &HashMap<i32, Vec<String>>,
  psize: usize,
) -> Vec<f64> {
  let mut out_vec = vec![0.; psize];
  for (_, i) in bined_hm.iter() {
    let length_count = i.len();
    if length_count < psize {
      out_vec[length_count - 1] = out_vec[length_count - 1] + 1.;
    }
  }
  return out_vec
}

fn cumsum(
  mut cum_vec: Vec<f64>,
) -> Vec<f64> {
  let mut cumulus = 0.;
  for i in 0..cum_vec.len() {
    cumulus = cumulus + cum_vec[i];
    cum_vec[i] = cumulus;
  }
  return cum_vec
}

pub fn thresholder(
  pop_reads: f64,
  chromosome_size: f64,
  false_discovery_tolerance: f64,
  read_hm: &HashMap<i32, Vec<String>>,
  psize: usize,
) -> usize {
  let lambda = pop_reads * BIN_SIZE as f64 / eff_genome_length;
  let eff_genome_length = effective_genome_length_calculator!(chromosome_size);
  let p_values = r_ppoisson(lambda, psize);

  let mut peak_prob = vec![0.; psize];
  for (ix, p_val) in p_values.iter().enumerate() {
    peak_prob[ix] = p_val * chromosome_size;
  }

  let bin_tb = tabler(read_hm, psize);
  let cum_bin_tb = cumsum(bin_tb);
  let mut false_disc_values = vec![0.; psize];
  for ix in 0..psize {
    false_disc_values[ix] = peak_prob[ix] / cum_bin_tb[ix];
  }

  let mut threshold = 0;
  for (ix, fd_val) in false_disc_values.iter().enumerate() {
    if *fd_val < false_discovery_tolerance {
      threshold = ix;
    }
  }
  return threshold;
}

// // R
// eff_genome_length <- length_post_contig * bin_size / bin_overlaps
//
// pop_reads <- sum(peak_strand[, "seq_sum"])
// lamm <- pop_reads * bin_size / eff_genome_length
// lambda_ls[[which_strand]] <- lamm
// p_values <- 1-ppois(1:(no_fdr), lambda = lamm)
//
// peak_prob <- p_values * length_post_contig
// false_disc_values <- peak_prob / cumsum(table(peak_strand[, "seq_max"]))[1:no_fdr]
// pop_thres_ls[[which_strand]] <- min(which(false_disc_values < false_discovery_tolerance))

////////////////////////////////////////////////////////////////////////////////////////////////////

// test private functions
#[cfg(test)]
mod tests {
  use data_test::data_test;
  use super::{
    ppois,
    effective_genome_length_calculator,
    r_ppoisson,
  };

  data_test! {
    // precission point => 20

    // test R ppois bindings
    fn test_ppois(sample, lambda, expected) => {
      let pois_vec: Vec<f64> = sample
        .iter()
        .map(|ix| unsafe {
            super::ppois(*ix, lambda, 1, 0)
          } )
        .collect();
      assert_eq!(pois_vec, expected)
    }
    - zero (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 0., vec![1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, ])
    - ichi (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 1., vec![0.36787944117144233402, 0.73575888234288466805, 0.91969860292860583506, 0.98101184312384615005, 0.99634015317265633982, 0.99940581518241833336, 0.99991675885071196195, 0.99998975080332530574, 0.99999887479740201535, 0.99999988857452171143, 0.99999998995223360332, ])
    - ni (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 2., vec![0.13533528323661270232, 0.40600584970983810695, 0.67667641618306340057, 0.85712346049854692964, 0.94734698265628891622, 0.98343639151938555543, 0.99546619447375117584, 0.99890328103214132138, 0.99976255267173885777, 0.99995350192498277941, 0.99999169177563151933, ])
    - san (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 3., vec![0.049787068367863944462, 0.19914827347145580561, 0.4231900811268435314, 0.64723188878223114617, 0.81526324452377196828, 0.91608205796869657256, 0.96649146469115876368, 0.98809549614364267089, 0.99619700793832399732, 0.99889751186988451348, 0.99970766304935265723, ])
    - yon (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 4., vec![0.018315638888734178669, 0.091578194443670934977, 0.23810330555354436433, 0.43347012036670901081, 0.62883693517987349075, 0.78513038703040516353, 0.8893260215974262417, 0.94886638420715263553, 0.97863656551201583245, 0.99186775720306608051, 0.99716023387948626855, ])
    - go (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 5., vec![0.0067379469990854670008, 0.040427681994512805475, 0.1246520194830811612, 0.26502591529736180265, 0.44049328506521229221, 0.61596065483306317034, 0.76218346297293870784, 0.86662832592999261561, 0.93190636527815151613, 0.96817194269379514004, 0.98630473140161711854, ])
    - roku (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 6., vec![0.0024787521766663584907, 0.017351265236664505098, 0.061968804416658973544, 0.15120388277664784105, 0.2850565003166312672, 0.44567964136461130087, 0.606302782412591168, 0.74397976045371705389, 0.84723749398456105197, 0.91607598300512427247, 0.95737907641746189391, ])
    - nana (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 7., vec![0.00091188196555451624375, 0.0072950557244361334194, 0.029636163880521784048, 0.08176541624472165315, 0.17299160788207135209, 0.30070827617436096668, 0.44971105584869880412, 0.59871383552303680808, 0.72909126773808230482, 0.83049593723867354278, 0.90147920588908725392, ])
    - hachi (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 8., vec![0.00033546262790251185322, 0.0030191636511226085764, 0.013753967744002993517, 0.042380111991684003836, 0.099632400487046052229, 0.19123606207962520753, 0.31337427753639757189, 0.45296080948699457558, 0.5925473414375912462, 0.71662425872701085439, 0.81588579255854631889, ])
    - ku (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 9., vec![0.00012340980408667956121, 0.0012340980408667970216, 0.0062321951063773160795, 0.021226486302908888215, 0.054963641495104915979, 0.11569052084105772848, 0.20678083985998707561, 0.32389696431289594081, 0.45565260432241877497, 0.58740824433194127607, 0.70598832034051173245, ])
    - juu (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 10., vec![4.5399929762484854173e-05, 0.00049939922738733333492, 0.0027693957155115757861, 0.010336050675925727987, 0.029252688076961082253, 0.067085962879031804662, 0.1301414208824830665, 0.22022064660169907158, 0.33281967875071877261, 0.45792971447185226719, 0.58303975019298537319, ])

    fn test_effective_genome_length_calculator(glen, expected) => {
      assert_eq!(super::effective_genome_length_calculator(glen, super::BIN_SIZE as f64, super::BIN_OVERLAP as f64), expected)
    }
    - dosmil (2000., 4000., )
    - dizmil (3243556456., 6487112912., )
    // test inverted probability poisson function
    fn test_r_poisson(lambda, psize, expected) => {
      assert_eq!(super::r_ppoisson(lambda, psize), expected)
    }
    - zero (0., 20, vec![0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, ])
    - un (1., 20, vec![0.26424111765711533195, 0.080301397071394164939, 0.01898815687615384995, 0.0036598468273436601805, 0.00059418481758166663553, 8.3241149288038052134e-05, 1.0249196674694260878e-05, 1.1252025979846536075e-06, 1.114254782885737427e-07, 1.0047766396681367951e-08, 8.3161078023863410635e-10, 6.359779369802254223e-11, 4.5198289555514747917e-12, 2.9998226125371729722e-13, 1.8651746813702629879e-14, 1.1102230246251565404e-15, 1.1102230246251565404e-16, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, ])
    - deux (2., 20, vec![0.59399415029016189305, 0.32332358381693659943, 0.14287653950145307036, 0.052653017343711083775, 0.016563608480614444574, 0.0045338055262488241581, 0.0010967189678586786172, 0.00023744732826114223201, 4.6498075017220585892e-05, 8.3082243684806655892e-06, 1.3646151596491407076e-06, 2.0734695815871617697e-07, 2.9305696425119265314e-08, 3.8712304473165204399e-09, 4.7996828733687380009e-10, 5.6060489583842354477e-11, 6.1890492730753976502e-12, 6.4770411256631632568e-13, 6.4392935428259079345e-14, 6.1062266354383609723e-15, ])
    - trois (3., 20, vec![0.80085172652854419439, 0.5768099188731564686, 0.35276811121776885383, 0.18473675547622803172, 0.083917942031303427441, 0.033508535308841236322, 0.011904503856357329106, 0.0038029920616760026775, 0.0011024881301154865199, 0.0002923369506473427748, 7.1386628974212662513e-05, 1.6149048555957890017e-05, 3.4019146132324706855e-06, 6.703859112278109933e-07, 1.2408017080467459436e-07, 2.1647844516969882989e-08, 3.571551610015433198e-09, 5.5883619953789320789e-10, 8.3144269247270585765e-11, 1.1790457499216699944e-11, ])
    - quatre (4., 20, vec![0.90842180555632912053, 0.76189669444645558016, 0.56652987963329093368, 0.37116306482012650925, 0.21486961296959483647, 0.1106739784025737583, 0.051133615792847364467, 0.021363434487984167554, 0.0081322427969339194931, 0.0028397661205137314511, 0.00091522914727004689439, 0.0002737168228554853755, 7.6328415343329680809e-05, 1.993172748271376804e-05, 4.8926107198976609425e-06, 1.1328315291381230168e-06, 2.4817760191364612865e-07, 5.158784033287844295e-08, 1.0200522093661845702e-08, 1.923058490227447237e-09, ])
    - cinq (5., 20, vec![0.95957231800548714595, 0.87534798051691886656, 0.73497408470263825286, 0.55950671493478765228, 0.38403934516693682966, 0.23781653702706129216, 0.13337167407000738439, 0.068093634721848483871, 0.031828057306204859955, 0.013695268598382881464, 0.0054530919130093558067, 0.0020188516274370904569, 0.00069798997913994575981, 0.00022625367617679081889, 6.9008241855628149608e-05, 1.9869043630382776655e-05, 5.4163382700034290451e-06, 1.4016978920894374028e-06, 3.4521358205363839033e-07, 8.1092504600199788456e-08, ])
    - six (6., 20, vec![0.98264873476333547409, 0.93803119558334102646, 0.8487961172233521312, 0.71494349968336878831, 0.55432035863538864362, 0.393697217587408832, 0.25602023954628294611, 0.15276250601543894803, 0.083924016994875727526, 0.042620923582538106089, 0.020091963539444757103, 0.0088274835178981936323, 0.0036284927387227883244, 0.0014003538333619003353, 0.0005090982712175895486, 0.000174877435413445248, 5.6917140423773382452e-05, 1.7597042093808745733e-05, 5.1801689370245540545e-06, 1.4551069900115010114e-06, ])
    - sept (7., 20, vec![0.99270494427556388306, 0.97036383611947818473, 0.91823458375527833297, 0.82700839211792864791, 0.69929172382563908883, 0.5502889441513012514, 0.40128616447696319192, 0.27090873226191769518, 0.16950406276132645722, 0.098520794110912746078, 0.053349623151558667189, 0.026999773425268713822, 0.012811392803420251774, 0.0057172024924960762604, 0.0024065803473980462712, 0.00095818315891771366211, 0.00036178431660227605704, 0.00012985143347954419824, 4.4402476539584512238e-05, 1.4495341610687439982e-05, ])
    - huit (8., 20, vec![0.99698083634887735283, 0.98624603225599705159, 0.95761988800831598923, 0.90036759951295397553, 0.80876393792037482022, 0.68662572246360242811, 0.54703919051300542442, 0.4074526585624087538, 0.28337574127298914561, 0.18411420744145368111, 0.11192400101851851524, 0.063797196736561923025, 0.03418070179381937912, 0.017256990397966465167, 0.0082310109868448666504, 0.0037180212812841784142, 0.0015942614198437565776, 0.00065036814809249499092, 0.00025293940209192289359, 9.3967903691760668039e-05, ])
    - neuf (9., 20, vec![0.99876590195913317327, 0.99376780489362270821, 0.97877351369709109097, 0.94503635850489509096, 0.88430947915894231315, 0.79321916014001292439, 0.67610303568710405919, 0.54434739567758128054, 0.41259175566805872393, 0.29401167965948826755, 0.19699161747065740968, 0.124226570829035321, 0.073850769307911789952, 0.041466325472903742266, 0.022035659171898980269, 0.011105909377583822462, 0.0053195712511816539703, 0.0024264021879805142135, 0.0010559536843589567567, 0.00043925185772930586126, ])
    - dix (10., 20, vec![0.99950060077261271285, 0.99723060428448839776, 0.98966394932407431018, 0.97074731192303886917, 0.93291403712096820922, 0.86985857911751696125, 0.77977935339830095618, 0.6671803212492812829, 0.54207028552814773281, 0.41696024980701462681, 0.3032238536968934195, 0.20844352360512574673, 0.13553557738068877647, 0.083458472934663019416, 0.048740403303978663274, 0.027041609784801079464, 0.014277613597049598759, 0.0071865046038542823581, 0.0034543419758568116862, 0.0015882606618580208391, ])
  }
