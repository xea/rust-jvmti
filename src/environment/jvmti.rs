use super::super::version::VersionNumber;

pub trait JVMTI {

    ///
    /// Return the JVM TI version number, which includes major, minor and micro version numbers.
    ///
    fn get_version_number(&self) -> VersionNumber;
}
