use lazy_static::lazy_static;
use qlty_analysis::version::{BUILD_PROFILE, GIT_COMMIT_OID, QLTY_VERSION};

lazy_static! {
    pub static ref BUILD_IDENTIFIER: String = match BUILD_PROFILE {
        "release" => format!("({})", GIT_COMMIT_OID),
        _ => format!("({} {})", GIT_COMMIT_OID, BUILD_PROFILE),
    };
    pub static ref LONG_VERSION: String = format!("{} {}", QLTY_VERSION, BUILD_IDENTIFIER.as_str());
}
