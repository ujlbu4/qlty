use lazy_static::lazy_static;

pub const QLTY_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_COMMIT_OID: &str = env!("GIT_COMMIT_OID");
pub const BUILD_PROFILE: &str = env!("BUILD_PROFILE");

lazy_static! {
    pub static ref BUILD_IDENTIFIER: String = match BUILD_PROFILE {
        "release" => format!("({})", GIT_COMMIT_OID),
        _ => format!("({} {})", GIT_COMMIT_OID, BUILD_PROFILE),
    };
    pub static ref LONG_VERSION: String = format!("{} {}", QLTY_VERSION, BUILD_IDENTIFIER.as_str());
}
