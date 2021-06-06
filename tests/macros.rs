/// Creates a new test case for varying use cases
///
/// # Examples
///
/// ## Used for creating command line tests
/// ```
/// chapulin_test!(cli => r#"
/// [scripts.example]
/// alias = "example"
/// command = "echo example"
/// "#| _cfg: ChildPath, mut cmd: Command | {
///     ... do some cli tests here
/// });
/// ```
///
/// ## Used for creating tests using the library
/// ```
/// chapulin_test!(lib => r#"
/// [scripts.example]
/// alias = "example"
/// command = "echo example"
/// "#| _cfg: ChildPath, mut lib: Config | {
///     ... do some library tests here
/// });
/// ```
///
/// ## Used for creating tests which need more control over the base parameters
/// ```
/// chapulin_test!(basic => r#"
/// [scripts.example]
/// alias = "example"
/// command = "echo example"
/// "#| _cfg: ChildPath, mut lib: Config | {
///     ... do some tests here
/// });
/// ```
#[macro_export]
macro_rules! chapulin_test {
    (basic => $name:ident, $func:expr) => {
        #[test]
        fn $name() {
            let dir = crate::common::TestEnv::new();
            $func(dir)
        }
    };
}
