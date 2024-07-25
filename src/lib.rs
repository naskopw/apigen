pub mod error;
pub mod plugins;

pub type Result<T> = std::result::Result<T, error::Error>;

use crate::error::Error;
use std::path::Path;

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<oas3::Spec> {
    match oas3::from_path(path) {
        Ok(spec) => Ok(spec),
        Err(err) => Err(Error::from(err)),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::plugins::PluginType;

    #[test]
    fn test_plugin_type_from_str() {
        assert_eq!(
            PluginType::from_str("path").unwrap(),
            PluginType::FromSysPath
        );
        assert_eq!(
            PluginType::from_str("bin").unwrap(),
            PluginType::CustomBinary
        );
        assert!(PluginType::from_str("foo").is_err());
    }
}
