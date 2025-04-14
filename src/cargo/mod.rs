//!
//! Sub-modules for dealing with cargo environment variables.
//! 
//! <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
//! 

pub mod crates;

#[cfg(test)]
mod crates_tests
{

    use super::crates::*;
    
    #[test]
    fn all_single_results()
    {

        println!("{}\n", cargo_env());

        println!("{}\n", cargo_manifest_dir_env());

        println!("{}\n", cargo_manifest_path_env());
        
        println!("{}\n", cargo_pkg_version_env());

        println!("{}\n", cargo_pkg_version_major_env());

        println!("{}\n", cargo_pkg_version_minor_env());

        println!("{}\n", cargo_pkg_version_pre_env());

        println!("{}\n", cargo_pkg_authors_env());

        println!("{}\n", cargo_pkg_name_env());

        println!("{}\n", cargo_pkg_description_env());

        println!("{}\n", cargo_pkg_homepage_env());

        println!("{}\n", cargo_pkg_repository_env());

        println!("{}\n", cargo_pkg_license_env());

        println!("{}\n", cargo_pkg_license_file_env());

        println!("{}\n", cargo_pkg_rust_version_env());

        println!("{}\n", cargo_pkg_readme_env());

    }
    
}
