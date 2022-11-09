mod test_helpers;

use anyhow::Result;
use vcard_compact::{parameter::*, parse, types::*, Error};

#[test]
fn error_empty() -> Result<()> {
    let result = parse("");
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_wrong_token() -> Result<()> {
    let result = parse("VERSION:4.0");
    assert!(matches!(result, Err(Error::IncorrectToken)));
    Ok(())
}

#[test]
fn error_no_version() -> Result<()> {
    let input = r#"BEGIN:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_no_end() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_parse_from_str() -> Result<()> {
    assert!(parse_boolean("foo").is_err());

    assert!("foo".parse::<TelephoneType>().is_err());
    assert!("foo".parse::<RelatedType>().is_err());
    assert!("foo".parse::<ValueType>().is_err());

    assert!("0;urn:uid:".parse::<ClientPidMap>().is_err());

    Ok(())
}

#[test]
fn error_type_unknown() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN;TYPE=FOO:Jane Doe
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::UnknownRelatedType(_))));
    Ok(())
}

#[test]
fn error_parse_token_expected() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_parse_delimiter_expected() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN\,"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::DelimiterExpected)));
    Ok(())
}

#[test]
fn error_parse_pref_out_of_range() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN;PREF=0:Jane Doe"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::PrefOutOfRange(_))));
    Ok(())
}

#[test]
fn error_parse_type_on_invalid_prop() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
PRODID;TYPE=home:urn:uid:
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::TypeParameter(_))));
    Ok(())
}

#[test]
fn error_parse_geo_not_quoted() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN;GEO=geo:1,2:Jane Doe
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::NotQuoted(_))));
    Ok(())
}

#[test]
fn error_parse_label_bad_prop() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN;LABEL=Jane:Jane Doe
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::InvalidLabel(_))));
    Ok(())
}

#[test]
fn error_parse_quoted_eof() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN;GEO="urn:""#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_parse_parameter_eof() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN;GEO="#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_parse_quoted_delimiter_expected() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN;GEO="urn:"\,"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::DelimiterExpected)));
    Ok(())
}

#[test]
fn error_parse_name_only_once() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
N:Jane
N:Doe
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::OnlyOnce(_))));
    Ok(())
}

#[test]
fn error_parse_bday_only_once() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
BDAY:--0203
BDAY:--0203
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::OnlyOnce(_))));
    Ok(())
}

#[test]
fn error_parse_anniversary_only_once() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
ANNIVERSARY:--0203
ANNIVERSARY:--0203
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::OnlyOnce(_))));
    Ok(())
}

#[test]
fn error_parse_gender_only_once() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
GENDER:M
GENDER:F
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::OnlyOnce(_))));
    Ok(())
}

#[test]
fn error_parse_prodid_only_once() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
PRODID:Foo
PRODID:Foo
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::OnlyOnce(_))));
    Ok(())
}

#[test]
fn error_parse_rev_only_once() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
REV:19951031T222710Z
REV:19951031T222710Z
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::OnlyOnce(_))));
    Ok(())
}

#[test]
fn error_parse_uid_only_once() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
UID:foo
UID:foo
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::OnlyOnce(_))));
    Ok(())
}

#[test]
fn error_parse_client_pid_map_with_pid() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
CLIENTPIDMAP;PID=1.1:1;urn:uuid:3df403f4-5924-4bb7-b077-3c711d9eb34b
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::ClientPidMapPidNotAllowed)));

    // Trigger an else branch
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
CLIENTPIDMAP;PREF=1;urn:uuid:3df403f4-5924-4bb7-b077-3c711d9eb34b
END:VCARD"#;
    let _ = parse(input);

    Ok(())
}

#[test]
fn error_parse_version_misplaced() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
VERSION:4.0
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::VersionMisplaced)));

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
VERSION:3.0
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::VersionMisplaced)));
    Ok(())
}

#[test]
fn error_parse_tz_value_type() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
TZ;VALUE=date-time:Rayleigh/North America
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::UnsupportedValueType(_, _))));
    Ok(())
}

#[test]
fn error_parse_date_time_text_value_type() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
BDAY;VALUE=uri:https://example.com
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::UnsupportedValueType(_, _))));
    Ok(())
}
