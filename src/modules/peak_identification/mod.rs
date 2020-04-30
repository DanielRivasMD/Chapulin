
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
// use rayon::prelude::*;

// crate utilities
use crate::utils::{
  read_record::ReadRecord
};

// modules
mod pi_mapping;

pub fn pi_controller(
  // key: &String,
  hash_map_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

// pub fn pi_controller(
//   hash_map_collection: &HashMap<String, ReadRecord>,
//   hash_map_anchor: &HashMap<String, Vec<String>>,
// ) -> std::io::Result<()> {

  let mut tmp_vec = Vec::new();
  for okey in hash_map_anchor.lock().unwrap().keys() {
    let ckey = okey.clone();
    tmp_vec.push(ckey);
  }

  for okey in tmp_vec {

    let c_hash_map_collection = hash_map_collection.clone();
    let c_hash_map_anchor = hash_map_anchor.clone();
    println!("{}", okey);

    let pi_handle = thread::spawn(move || {

    pi_mapping::pi_identifier(
      &okey,
      c_hash_map_collection,
      c_hash_map_anchor, 
    ).expect(&okey);

    // if let Some(current_chr) = c_pi_anchor_registry.lock().unwrap().get(i) {
    //   println!("This is a chromosome set: {}\n{:#?}", i, current_chr);
    // }

    });

    pi_handle.join().unwrap();

  }

  // if let Some(read_vec) = hash_map_anchor.lock().unwrap().get(key) {
  //   for chr_read in read_vec {
  //     if let Some(read_record) = hash_map_collection.lock().unwrap().get(chr_read) {
  //       println!("{:#?}", read_record);
  //     }
  //
  //   }
  //
  // }

  // TODO: divide by chromosomes to create concurrency

  // TODO: also divide by orientation & position

  // TODO: bin choromosomal positions to accumulate peaks

  // let concurrent_fn = |input: &HashMap<String, Vec<String>>| -> &str {
  //   let _: i32 = input.par_iter()
  //     .map(|i| {
  //       println!("{:?}", i);
  //       for j in i.1.iter() {
  //         let tmp_entry = hash_map_collection.get(j);
  //         println!("{:#?}", tmp_entry);
  //       }
  //       1 as i32 * 1 as i32
  //       // i.1.iter()
  //       //   .map(|j| {
  //       //     println!("{:#?}", hash_map_collection.get(j));
  //       // });
  //     })
  //     .sum();
  //   return "Done\n"
  // };
  //
  // let con_out = concurrent_fn(hash_map_anchor);
  // println!("{}", con_out);

  // let concurrent_out = hash_map_anchor.par_iter()
  //   .map(|i| -> () {
  //     println!("Concurrent: {:?}", i.0);
  //   });
  //
  // println!("{:#?}", concurrent_out);

// let mut keys_vec = Vec::new();

  // let mut count = 0;
  // for (ak, _av) in hash_map_anchor.lock().unwrap().iter() {
  //
  //   let c_hm_anchor = hash_map_anchor.clone();
  //   if let Some(c_reads) = c_hm_anchor.lock().unwrap().get(ak) {
  //
  //     println!("This is a chromosome set: {}\n{:#?}", ak, c_reads);
  //   }

    // for i in c_reads {
    //   println!("{:#?}", hash_map_collection.lock().unwrap().get(i));
    // }

    // for (ck, cv) in hash_map_collection.lock().unwrap().iter() {
    //   if cv.read1.chr_read == c_hm_anchor.lock().unwrap().
    // }

    // keys_vec[count] = *i.clone();
    // count = count + 1;
    // keys_vec.push(i.clone());
  // }

  // println!("{:#?}", keys_vec);

  // let mut glob_count = 0;
  // for (i, j) in hash_map_anchor.lock().unwrap().iter() {
  //   println!("Sequential: {:?} => {}", i, j.len());
  //   glob_count = glob_count + j.len();
  // }
  // println!("Size of anchors: {}", glob_count);
  // println!("Size of collection: {}", hash_map_collection.lock().unwrap().len());

  // for (k_str, v_vec) in hash_map_anchor.iter().par_bridge() {
  //   println!("{} => {:#?}", k_str, v_vec);
  // }
  // let mut counter = 0;
  // for (_key, val) in hash_map_collection.iter() {
  //   // println!("key: {}\nval: {:#?}", key, val);
  //
  //   if val.read1.chr_read[0].chr == "chr1".to_string() {
  //     counter = counter + 1;
  //     println!("{} => {}", val.read1.chr_read[0].chr, val.read1.chr_read[0].pos);
  //   }
  // }
  // println!("{}", counter);
  // TODO: set up a threshold based on the poisson estimation of false discovery rate

  // TODO: gather all positions & output a comprenhensive list

  Ok(())
}



// # Load SAM-like flat file - Silently
// f_in_ind_file <- paste0(tmp_dir, which_vervet, "_chr", Chromosome, ".txt")
// suppressMessages( f_ind_var <- readr::read_tsv(f_in_ind_file) )
//
// f_count_dummy_file <- paste0(data_bin_dir, "chlSab_chr", Chromosome, ".RData")
// f_nombres <- as.vector(outer(which_vervet, sum_names, paste, sep = "_"))
//
// if ( which_vervet == names(vervet_ls[1]) & b_ervVchr == "ERV" ) {
// 	#
// 	f_x_max <- max(f_ind_var[, "Chromosomal_Position"]) + (pos_margin * bin_size)
// 	Positions <- seq(from = 0, to = f_x_max, by = bin_size / bin_overlaps)
// 	count_matrix <- tibble::tibble(Chr = Chromosome, Positions = Positions)
// 	save(count_matrix, file = f_count_dummy_file)
// 	cat("Save count dummy", fill = TRUE)
// } else {
// 	#
// 	load(f_count_dummy_file)
// 	Positions <- count_matrix$Positions
// 	cat("Load count dummy", fill = TRUE)
// }
//
// if ( b_ervVchr == "ERV" ){
// 	#
// 	col_tag <- "LTR_cand"
//
// 	f_ind_var <- Rpack.chlSab::read_orient(
// 		fi_col_tag = col_tag,
// 		fi_df = f_ind_var,
// 		fi_tags = strand_ori,
// 		fie_est_insert_size = est_insert_size,
// 		fie_forward_seq = forward_seq,
// 		fie_reverse_seq = reverse_seq,
// 		fie_forward_strand = forward_strand,
// 		fie_reverse_strand = reverse_strand,
// 		bi_ervVchr = b_ervVchr
// 	)
//
// 	# Eliminate identical chromosomal anchoring positions & supplementary alignments (ERV)
// 	f_ind_var <- f_ind_var[!duplicated(f_ind_var[c("Chromosomal_Position", col_tag)]), ]
// } else if ( b_ervVchr == "CHR" ) {
// 	#
// 	col_tag <- "read_cand"
//
// 	f_ind_var <- Rpack.chlSab::read_orient(
// 		fi_col_tag = col_tag,
// 		fi_df = f_ind_var,
// 		fi_tags = strand_ori,
// 		fie_forward_strand = forward_strand,
// 		fie_reverse_strand = reverse_strand,
// 		bi_ervVchr = b_ervVchr
// 	)
// }
//
// tmp_hit_df <- data.frame(hit_chr = paste0("chr", Chromosome), Positions = Positions)
// for ( strand in seq_along(f_nombres) ) {
// 	#
// 	tmp_hit_df[, f_nombres[strand]] <- 0
// 	hit_table <- CopperGenomicFunctions::concat_ls(
// 		CopperGenomicFunctions::slid_win_tov(
// 			f_ind_var[which(f_ind_var[, col_tag] == strand_ori[strand]), "Chromosomal_Position"]
// 		)
// 	)
// 	tmp_hit_pos <- match(names(hit_table), tmp_hit_df[, "Positions"])
// 	tmp_hit_df[tmp_hit_pos[which(!is.na(tmp_hit_pos))], f_nombres[strand]] <- hit_table[which(names(hit_table) >= 0)]
// }
// count_matrix[, f_nombres] <- tmp_hit_df[, f_nombres]
//
// return(count_matrix)
