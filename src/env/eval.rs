use std::env;
use std::str::FromStr;

pub fn eval_env_or<T>(var_name: &str, default_value: T) -> T
where
    T: FromStr,
{
    let Ok(var_value) = env::var(var_name) else {
        return default_value;
    };

    var_value.parse::<T>().unwrap_or(default_value)
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
            env::set_var("ENABLE", "true");
            assert_eq!(eval_env_or::<bool>("ENABLE", false), true);
        }
    }
}
