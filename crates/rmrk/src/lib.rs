#![cfg_attr(not(feature = "std"), no_std)]

mod config;

pub use config::Config;

pub mod error {
    pub use rmrk_common::error::*;
}

pub mod types {
    pub use rmrk_common::types::*;
}

pub mod storage {
    pub mod base {
        pub use rmrk_base::*;
    }

    pub mod minting {
        pub use rmrk_minting::*;
    }

    pub mod multiasset {
        pub use rmrk_multiasset::*;
    }

    pub mod nesting {
        pub use rmrk_nesting::*;
    }
}

pub mod traits {
    pub use rmrk_base::traits::*;
    pub use rmrk_minting::traits::*;
    pub use rmrk_multiasset::traits::*;
    pub use rmrk_nesting::traits::*;
}

pub mod util {
    pub use rmrk_common::utils::*;
}
