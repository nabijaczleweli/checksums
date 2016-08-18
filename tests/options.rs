mod depth {
    extern crate checksums;

    use self::checksums::options::DepthSetting;
    use std::str::FromStr;


    mod next_level {
        use self::super::checksums::options::DepthSetting;


        #[test]
        fn infinite() {
            assert_eq!(DepthSetting::Infinite.next_level(), Some(DepthSetting::Infinite));
        }

        #[test]
        fn last_level() {
            assert_eq!(DepthSetting::LastLevel.next_level(), None);
        }

        #[test]
        fn nremaining() {
            assert_eq!(DepthSetting::NRemaining(1).next_level(), Some(DepthSetting::LastLevel));

            assert_eq!(DepthSetting::NRemaining(2).next_level(), Some(DepthSetting::NRemaining(1)));
            assert_eq!(DepthSetting::NRemaining(100).next_level(), Some(DepthSetting::NRemaining(99)));
        }
    }

    #[test]
    fn from_str() {
        for p in &[("-1", DepthSetting::Infinite),
                   ("-100", DepthSetting::Infinite),
                   ("0", DepthSetting::LastLevel),
                   ("1", DepthSetting::NRemaining(1)),
                   ("2", DepthSetting::NRemaining(2)),
                   ("100", DepthSetting::NRemaining(100))] {
            assert_eq!(DepthSetting::from_str(p.0).unwrap(), p.1);
        }
    }

    #[test]
    fn from_str_bad() {
        for s in &["a234", "1231d"] {
            DepthSetting::from_str(s).unwrap_err();
        }
    }
}
