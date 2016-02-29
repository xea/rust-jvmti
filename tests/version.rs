extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::version::VersionNumber;

    #[test]
    fn valid_version_numbers_should_be_parsed_correctly() {
        let mut input: u32 = 0x00000000;
        let mut version = VersionNumber::from_u32(&input);

        assert_eq!(0, version.major_version);
        assert_eq!(0, version.minor_version);
        assert_eq!(0, version.micro_version);

        input = 0x00000010;
        version = VersionNumber::from_u32(&input);

        assert_eq!(0, version.major_version);
        assert_eq!(0, version.minor_version);
        assert_eq!(0x10, version.micro_version);

        input = 0x00002010;
        version = VersionNumber::from_u32(&input);

        assert_eq!(0, version.major_version);
        assert_eq!(0x20, version.minor_version);
        assert_eq!(0x10, version.micro_version);

        input = 0x00302010;
        version = VersionNumber::from_u32(&input);

        assert_eq!(0x30, version.major_version);
        assert_eq!(0x20, version.minor_version);
        assert_eq!(0x10, version.micro_version);

        input = 0x33214310;
        version = VersionNumber::from_u32(&input);

        assert_eq!(0x321, version.major_version);
        assert_eq!(0x43, version.minor_version);
        assert_eq!(0x10, version.micro_version);
    }
}
