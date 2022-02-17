use validator::Validate;

#[test]
fn can_validate_url_ok() {
    #[derive(Debug, Validate, Default)]
    struct TestStruct {
        #[validate(url)]
        val: String,
        #[validate(skip)]
        reserved: [u8; 11],
    }

    let s = TestStruct { val: "https://google.com".to_string(), ..Default::default() };

    assert!(s.validate().is_ok());
}
