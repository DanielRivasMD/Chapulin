
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

use chapulin::utils::functions::flag_interpretor::interpretor;

////////////////////////////////////////////////////////////////////////////////////////////////////


data_test! {

  fn test_interpretor(flag, digit, expected) => {
    assert_eq!(super::interpretor(flag, digit), expected);
  }




  - r_p_adeen (1, 1, true)
  - r_p_dva (177, 1, true)
  - r_p_tree (72, 1, false)

  - r_mp_adeen (2, 2, true)
  - r_mp_dva (135, 2, true)
  - r_mp_tree (177, 2, false)

  - r_unmp_adeen (4, 3, true)
  - r_unmp_dva (2161, 3, true)
  - r_unmp_tree (121, 3, false)

  - m_unmp_adeen (8, 4, true)
  - m_unmp_dva (185, 4, true)
  - m_unmp_tree (177, 4, false)

  - r_rev_st_adeen (16, 5, true)
  - r_rev_st_dva (157, 5, true)
  - r_rev_st_tree (169, 5, false)

  - m_rev_st_adeen (32, 6, true)
  - m_rev_st_dva (99, 6, true)
  - m_rev_st_tree (147, 6, false)

  - f_pair_adeen (64, 7, true)
  - f_pair_dva (73, 7, true)
  - f_pair_tree (133, 7, false)

  - s_pair_adeen (128, 8, true)
  - s_pair_dva (329, 8, true)
  - s_pair_tree (73, 8, false)

  - not_pr_align_adeen (256, 9, true)
  - not_pr_align_dva (389, 9, true)
  - not_pr_align_tree (133, 9, false)

  - r_fail_q_adeen (512, 10, true)
  - r_fail_q_dva (901, 10, true)
  - r_fail_q_tree (329, 10, false)

  - r_pcr_op_dup_adeen (1024, 11, true)
  - r_pcr_op_dup_dva (1157, 11, true)
  - r_pcr_op_dup_tree (133, 11, false)

  - suppl_alig_adeen (2048, 12, true)
  - suppl_alig_dva (3095, 12, true)
  - suppl_alig_tree (99, 12, false)
  
  

}

////////////////////////////////////////////////////////////////////////////////////////////////////
