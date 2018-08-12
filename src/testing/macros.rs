//! Testing macros
#![allow(unused)]


macro_rules! test_if {
    ($cfg_tt:tt: $it:item) => {
        #[cfg(any(
            // Test everything if:
            //
            // * tests are enabled,
            // * no features about exclusively testing
            //   specific vector classes are enabled
            all(test, not(any(
                feature = "test_v16",
                feature = "test_v32",
                feature = "test_v64",
                feature = "test_v128",
                feature = "test_v256",
                feature = "test_v512",
                feature = "test_none",
            ))),
            // Test if:
            //
            // * tests are enabled
            // * a particular cfg token tree returns true
            all(test, feature = $cfg_tt),
        ))]
        $it
    };
}

macro_rules! impl_ {
    ($($it:item)*) => {
        // Impl if:
        //
        // * the feature disable_impls is not available
        $(
            #[cfg(not(feature = "disable_impls"))]
            $it
        )*
    };
}
