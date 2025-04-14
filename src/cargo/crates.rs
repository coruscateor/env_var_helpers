//!
//! This module contains functions which return environment variable values that cargo sets for crates.
//! 
//! <https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates>
//! 

use paste::paste;

//use casey::lower;

use crate::{impl_env_accessor, impl_env_accessor_pair};

//https://nrsk.dev/articles/capturing-doc-comments

/*
///
/// Fetches the value associated with the $name environment variable.
/// 
 */

//https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates

impl_env_accessor!(cargo, CARGO); 

impl_env_accessor_pair!(cargo, CARGO);



impl_env_accessor!(cargo_manifest_dir, CARGO_MANIFEST_DIR);

impl_env_accessor_pair!(cargo_manifest_dir, CARGO_MANIFEST_DIR);



impl_env_accessor!(cargo_manifest_path, CARGO_MANIFEST_PATH);

impl_env_accessor_pair!(cargo_manifest_path, CARGO_MANIFEST_PATH);



impl_env_accessor!(cargo_pkg_version, CARGO_PKG_VERSION);

impl_env_accessor_pair!(cargo_pkg_version, CARGO_PKG_VERSION);



impl_env_accessor!(cargo_pkg_version_major, CARGO_PKG_VERSION_MAJOR);

impl_env_accessor_pair!(cargo_pkg_version_major, CARGO_PKG_VERSION_MAJOR);



impl_env_accessor!(cargo_pkg_version_minor, CARGO_PKG_VERSION_MINOR);

impl_env_accessor_pair!(cargo_pkg_version_minor, CARGO_PKG_VERSION_MINOR);



impl_env_accessor!(cargo_pkg_version_patch, CARGO_PKG_VERSION_PATCH);

impl_env_accessor_pair!(cargo_pkg_version_patch, CARGO_PKG_VERSION_PATCH);



impl_env_accessor!(cargo_pkg_version_pre, CARGO_PKG_VERSION_PRE);

impl_env_accessor_pair!(cargo_pkg_version_pre, CARGO_PKG_VERSION_PRE);



impl_env_accessor!(cargo_pkg_authors, CARGO_PKG_AUTHORS);

impl_env_accessor_pair!(cargo_pkg_authors, CARGO_PKG_AUTHORS);



impl_env_accessor!(cargo_pkg_name, CARGO_PKG_NAME);

impl_env_accessor_pair!(cargo_pkg_name, CARGO_PKG_NAME);



impl_env_accessor!(cargo_pkg_description, CARGO_PKG_DESCRIPTION);

impl_env_accessor_pair!(cargo_pkg_description, CARGO_PKG_DESCRIPTION);



impl_env_accessor!(cargo_pkg_homepage, CARGO_PKG_HOMEPAGE);

impl_env_accessor_pair!(cargo_pkg_homepage, CARGO_PKG_HOMEPAGE);



impl_env_accessor!(cargo_pkg_repository, CARGO_PKG_REPOSITORY);

impl_env_accessor_pair!(cargo_pkg_repository, CARGO_PKG_REPOSITORY);



impl_env_accessor!(cargo_pkg_license, CARGO_PKG_LICENSE);

impl_env_accessor_pair!(cargo_pkg_license, CARGO_PKG_LICENSE);



impl_env_accessor!(cargo_pkg_license_file, CARGO_PKG_LICENSE_FILE);

impl_env_accessor_pair!(cargo_pkg_license_file, CARGO_PKG_LICENSE_FILE);



impl_env_accessor!(cargo_pkg_rust_version, CARGO_PKG_RUST_VERSION);

impl_env_accessor_pair!(cargo_pkg_rust_version, CARGO_PKG_RUST_VERSION);



impl_env_accessor!(cargo_pkg_readme, CARGO_PKG_README);

impl_env_accessor_pair!(cargo_pkg_readme, CARGO_PKG_README);



//impl_cargo_env_accessor!(cargo_crate_name, CARGO_CRATE_NAME);

//impl_cargo_env_accessor!(cargo_bin_name, CARGO_BIN_NAME);

//impl_cargo_env_accessor!(out_dir, OUT_DIR);



/* 
impl_pub_env_accessor!
{

    ///
    /// Fetches the value associated with the CARGO envionement variable.
    /// 
    (cargo, CARGO)

}
*/
