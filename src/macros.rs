//use casey::lower;

//env

///
/// Creates a function that calls the env macro on the provided env_variable_name parameter returning its value.
/// 
/// Requires that the paste function of the paste crate be in module scope (paste::paste). 
/// 
/// The resultant function name is the provided fn_name_start identity value with "_env" appended.
/// 
#[macro_export]
macro_rules! impl_env_accessor
{

    ($fn_name_start:ident, $env_variable_name:ident) =>
    {

        paste!
        {

            #[doc = "Fetches the value associated with the "]
            #[doc = stringify!($env_variable_nam)]
            #[doc = " environment variable."]
            pub fn [<$fn_name_start _env>]() -> &'static str
            {

                env!(stringify!($env_variable_name))

            }

        }

    }

}


/*
#[macro_export]
macro_rules! impl_env_accessor
{

    ($env_variable_name:ident) =>
    {

        let $lower_env_variable_name = lower!($env_variable_name);

        paste!
        {

            #[doc = "Fetches the value associated with the "]
            #[doc = stringify!($env_variable_nam)]
            #[doc = " environment variable."]
            pub fn [<$lower_env_variable_name _env>]() -> &'static str
            {

                env!(stringify!($env_variable_name))

            }

        }

    }

}
*/

///
/// Creates a function that calls the env macro on the provided env_variable_name parameter returning the stringafied env_variable_name and the environment variable value in a tuple.
/// 
/// Requires that the paste function of the paste crate be in module scope (paste::paste). 
/// 
/// The resultant function name is the provided fn_name_start identity value with "_env" appended.
/// 
#[macro_export]
macro_rules! impl_env_accessor_pair
{

    ($fn_name_start:ident, $env_variable_name:ident) =>
    {

        paste!
        {

            #[doc = "Fetches the value associated with the "]
            #[doc = stringify!($env_variable_name)]
            #[doc = " environment variable. Returns a tuple with the name of the environment variable used in the first position and the value retrieved in the second position."]
            pub fn [<$fn_name_start _env_pair>]() -> (&'static str, &'static str)
            {

                (stringify!($env_variable_name), env!(stringify!($env_variable_name)))

            }

        }

    }

}
