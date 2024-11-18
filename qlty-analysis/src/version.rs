pub const QLTY_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_COMMIT_OID: &str = env!("GIT_COMMIT_OID");
pub const BUILD_PROFILE: &str = env!("BUILD_PROFILE");

pub fn qlty_semver() -> semver::Version {
    semver::Version::parse(QLTY_VERSION).expect("QLTY_VERSION should be a valid semver")
}
