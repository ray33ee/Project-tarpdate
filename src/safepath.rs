use std::path::{PathBuf, Path, Component};
use std::convert::TryFrom;
use serde::{Serialize, Deserialize, Deserializer, de};
use std::borrow::Borrow;

///A wrapper around Path of PathBuf that prevents invalid or unsafe archive paths (like the infamous '../' component)
#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub (in crate) struct SafePathBuf {
    path: PathBuf,
}

impl<'de> Deserialize<'de> for SafePathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let path = PathBuf::deserialize(deserializer);

        match path {
            Ok(k) => {
                SafePathBuf::try_from(k).map_err(|x| de::Error::custom(x))
            }
            Err(e) => {
                Err(e)
            }
        }

    }
}

impl TryFrom<PathBuf> for SafePathBuf {
    type Error = crate::error::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if let Err(e) = Self::verify(path.as_path()) {
            return Err(e);
        }

        Ok(Self {
            path,
        })
    }
}

impl TryFrom<&Path> for SafePathBuf {
    type Error = crate::error::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if let Err(e) = Self::verify(path) {
            return Err(e);
        }

        Ok(Self {
            path: PathBuf::from(path),
        })
    }
}

impl Borrow<Path> for SafePathBuf {
    fn borrow(&self) -> &Path {
        self.path.as_path()
    }
}

impl SafePathBuf {
    fn verify(path: &Path) -> crate::error::Result<()> {

        for component in path.components() {
            if let Component::ParentDir = component {
                return Err(crate::error::Error::new(crate::error::ErrorKind::UnsafePath(PathBuf::from(path)), format!("Illegal parent directory (..) found in path ({}).", path.to_str().unwrap())))
            }
        }

        Ok(())
    }

    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }
}
