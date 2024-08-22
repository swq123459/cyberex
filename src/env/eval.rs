use std::env;
use std::str::FromStr;

pub fn eval_env<T>(var_name: &str, default_value: T) -> Result<T, T::Err>
where
    T: FromStr,
{
    let Ok(var_value) = env::var(var_name) else {
        return Ok(default_value);
    };

    let parsed = var_value.parse::<T>()?;
    Ok(parsed)
}

pub fn eval_env_or<T>(var_name: &str, default_value: T) -> T
where
    T: FromStr + Clone,
{
    // Note: here ignore the parsed error
    eval_env(var_name, default_value.clone()).unwrap_or(default_value)
}

#[cfg(test)]
mod tests {
    use std::{
        net::SocketAddr,
        path::{Path, PathBuf},
    };

    use super::*;
    #[test]
    fn test_eval_env_or() {
        {
            let def = SocketAddr::from(([127, 0, 0, 1], 5423));
            assert_eq!(eval_env_or::<SocketAddr>("LC_ADDR", def), def);
        }

        {
            let def = Path::new("/root/path").to_path_buf();
            assert_eq!(eval_env_or::<PathBuf>("LC_KEY_DIR", def.clone()), def);
        }
        {
            unsafe { env::set_var("ENABLE", "true") };
            assert!(eval_env_or::<bool>("ENABLE", false));
        }
        {
            unsafe { env::set_var("LC_ADDR", "localhost:1922") };
            let def = SocketAddr::from(([127, 0, 0, 1], 5423));
            // localhost is no ip
            assert!(eval_env::<SocketAddr>("LC_ADDR", def).is_err());
        }
    }
}
