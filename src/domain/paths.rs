use std::borrow::Borrow;
use std::ops::Deref;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DirPath(PathBuf);

impl DirPath {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn into_inner(self) -> PathBuf {
        self.0
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> DirPath {
        DirPath(self.0.join(path))
    }

    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    pub fn canonicalize(&self) -> std::io::Result<DirPath> {
        self.0.canonicalize().map(DirPath)
    }

    pub fn starts_with(&self, other: &DirPath) -> bool {
        self.0.starts_with(&other.0)
    }
}

impl From<PathBuf> for DirPath {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}

impl AsRef<Path> for DirPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for DirPath {
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<Path> for DirPath {
    fn borrow(&self) -> &Path {
        &self.0
    }
}

impl std::fmt::Display for DirPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IsoPath(PathBuf);

impl IsoPath {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl From<PathBuf> for IsoPath {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}

impl AsRef<Path> for IsoPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for IsoPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<Path> for IsoPath {
    fn borrow(&self) -> &Path {
        &self.0
    }
}

impl std::fmt::Display for IsoPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DevicePath(PathBuf);

impl DevicePath {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl From<PathBuf> for DevicePath {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}

impl AsRef<Path> for DevicePath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for DevicePath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<Path> for DevicePath {
    fn borrow(&self) -> &Path {
        &self.0
    }
}

impl std::fmt::Display for DevicePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
