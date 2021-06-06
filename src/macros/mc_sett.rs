
////////////////////////////////////////////////////////////////////////////////////////////////////

// macro_rules! settings {
//   ($matc: expr) => {

//     // let now = SystemTime::now();

//     config = $matc.value_of("config")
//       .context(ChapulinConfigError::EmptyConfigOption)?;

//     let mut settings = Config::default();
//     settings
//       .merge(File::with_name(config))
//       .context(ChapulinConfigError::NoConfigFile)?;

//     // interpret settings into variables
//     let settings_hm = settings.try_into::<HashMap<String, String>>()
//       .context(ChapulinConfigError::ConfigHashMap)?;

//     // let directory = settings_hm.get("directory")
//     //   .context(ChapulinConfigError::BadDirectoryVar)?;
//     // let reference_file = settings_hm.get("reference")
//     //   .context(ChapulinConfigError::BadReferenceVar)?;
//     // let me_library_file = settings_hm.get("mobile_element_library")
//     //   .context(ChapulinConfigError::BadMELibVar)?;
//     // let me_align = settings_hm.get("mobile_element_alignment")
//     //   .context(ChapulinConfigError::BadMEAlignVar)?;
//     // let cl_align = settings_hm.get("reference_genome_alignment")
//     //   .context(ChapulinConfigError::BadReferenceGenomeVar)?;
//   };

// }

// ////////////////////////////////////////////////////////////////////////////////////////////////////
