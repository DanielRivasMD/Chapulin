
// Tests that it returns the error AliasNotFound if the alias given does not exist
// chapulin_test!(lib => test_error_alias_not_found, cfg => r#"
// [scripts.test_cmd_1]
// alias = 'test_cmd_1'
// command = 'echo test_1'
// "#, | _cfg: ChildPath, mut lib: Pier | {
    // err_eq!(lib.remove_script("non_existant"), AliasNotFound);
    // err_eq!(lib.fetch_script("non_existant"), AliasNotFound);
// });

// #[macro_export]
// macro_rules! chapulin_test {
//     (basic => $name:ident, $func:expr) => {
//         #[test]
//         fn $name() {
//             let dir = crate::common::TestEnv::new();
//             $func(dir)
//         }
//     };
// }

// TODO: add error handler tests
// #[cfg(test)]
// mod error {
//   #[test]
//   fn test_error() {
//
//   }
//
// }
