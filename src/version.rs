struct VersionOrder {
    fillvalue: u16,
    norm_version: String,
    local: Vec<String>,
    version: Vec<String>,
}

use version_compare::{CompOp, Version, VersionCompare};

#[cfg(test)]
mod tests {
    use version_compare::{CompOp, Version, VersionCompare};
    use pretty_assertions::{assert_eq, assert_ne};

    # [test]
    fn test_less_specific_less_than_more_specific() {
        // 0.4 < 0.4.0
        let a = Version::from("0.4");
        let b = Version::from("0.4.0");
        assert_eq!(a == b, true);
    }

    # [test]
    fn test_rc_greater_than_earlier_version_less_than_release() {
        // 0.4.0 < 0.4.1.rc < 0.4.1
        let a = Version::from("0.4.0");
        let b = Version::from("0.4.1.rc");
        let c = Version::from("0.4.1");
        assert_eq!(a < b, true);
        assert_eq!(b < c, true);
    }

    # [test]
    fn test_case_insensitive_rc() {
        // 0.4.1.rc == 0.4.1.RC
        let a = Version::from("0.4.1.rc");
        let b = Version::from("0.4.1.RC");
        assert_eq!(a == b, true);
    }

   # [test]
    fn test_lexicographical_sort_numbers() {
        // 0.5a1 < 0.5a2
        let a = Version::from("0.5a1");
        let b = Version::from("0.5a2");
       assert_eq!(a < b, true);
    }

    # [test]
    fn test_lexicographical_sort() {
        // 0.5a2 < 0.5b1
        let a = Version::from("0.5a2");
        let b = Version::from("0.5b1");
        assert_eq!(a < b, true);
    }

    # [test]
    fn test_dev_special_case_horribleness() {
        // 1.0 < 1.1dev1 < 1.1a1 < 1.1.0dev1 == 1.1.dev1
        let a = Version::from("1.0");
        let b = Version::from("1.1dev1");
        let c = Version::from("1.1a1");
        let d = Version::from("1.1.0dev1");
        let e = Version::from("1.1.dev1");
        assert_eq!(a < b, true);
        assert_eq!(b < c, true);
        assert_eq!(c < d, true);
        assert_eq!(d == e, true);
    }

    # [test]
    fn test_rc_with_number() {
        let a = Version::from("1.1.dev1");
        let b = Version::from("1.1.a1");
        let c = Version::from("1.1.0rc1");
        let d = Version::from("1.1.0");
        assert_eq!(a < b, true);
        assert_eq!(b < c, true);
        assert_eq!(c < d, true);
    }

    # [test]
    fn test_post_gt_release() {
        let a = Version::from("1.1.0");
        let b = Version::from("1.1.0post1");
        let c = Version::from("1996.07.12");
        assert_eq!(a == b, false);
        assert_eq!(a > b, false);
        assert_eq!(a < b, true);
        assert_eq!(b < c, true);
    }

    # [test]
    fn test_epoch() {
        let a = Version::from("1996.07.12");
        let b = Version::from("1:0.4.1");
        let c = Version::from("1:3.4.1");
        let d = Version::from("2:0.4.1");
        assert_eq!(a < b, true);
        assert_eq!(b < c, true);
        assert_eq!(c < d, true);
    }

}
