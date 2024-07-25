use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum PluginType {
    FromSysPath,
    CustomBinary,
}

impl FromStr for PluginType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "path" => Ok(PluginType::FromSysPath),
            "bin" => Ok(PluginType::CustomBinary),
            _ => Err(format!("unknown plugin: {}", s)),
        }
    }
}

#[cfg(windows)]
pub fn plugin_name_from_sys_path(name: &str) -> String {
    format!("apigen-{}.exe", name)
}

#[cfg(not(windows))]
pub fn plugin_name_from_sys_path(name: &str) -> String {
    format!("apigen-{}", name)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn test_plugin_name_from_sys_path_windows() {
        assert_eq!(super::plugin_name_from_sys_path("foo"), "apigen-foo.exe");
    }

    #[test]
    #[cfg(not(windows))]
    fn test_plugin_name_from_sys_path_unix() {
        assert_eq!(super::plugin_name_from_sys_path("foo"), "apigen-foo");
    }

    #[test]
    fn test_plugin_type_from_str() {
        assert_eq!(
            super::PluginType::from_str("path").unwrap(),
            super::PluginType::FromSysPath
        );
        assert_eq!(
            super::PluginType::from_str("bin").unwrap(),
            super::PluginType::CustomBinary
        );
        assert!(super::PluginType::from_str("foo").is_err());
    }
}
