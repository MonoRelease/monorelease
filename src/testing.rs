macro_rules! my_test {
    (fn $name:ident() $(-> $ret:ty)? $body:block) => {
        #[test]
        #[tracing::instrument(err(Debug))]
        fn $name() $(-> $ret)? {
            $crate::logging::tests::init_test_logging();
            $body
        }
    };
}

pub(crate) use my_test;
