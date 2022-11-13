use anyhow::Result;
use proptest::prelude::*;
use time::UtcOffset;
use vcard4::{helper::*, property::DateAndOrTime};

#[test]
fn types_time_only() -> Result<()> {
    let (time, _) = parse_time("102200")?;
    assert_eq!("10:22:00.0", &time.to_string());

    let (time, _) = parse_time("1022")?;
    assert_eq!("10:22:00.0", &time.to_string());

    let (time, _) = parse_time("10")?;
    assert_eq!("10:00:00.0", &time.to_string());

    let (time, _) = parse_time("-2200")?;
    assert_eq!("0:22:00.0", &time.to_string());

    let (time, _) = parse_time("--00")?;
    assert_eq!("0:00:00.0", &time.to_string());

    let (time, _) = parse_time("102200Z")?;
    assert_eq!("10:22:00.0", &time.to_string());

    let (time, offset) = parse_time("102200-0800")?;
    assert_eq!("10:22:00.0", &time.to_string());
    assert_eq!("-08:00:00", &offset.to_string());

    // Trigger some branches
    assert!(parse_time("-").is_err());

    Ok(())
}

#[test]
fn types_date_only() -> Result<()> {
    let date = parse_date("19850412")?;
    assert_eq!("1985-04-12", &date.to_string());

    let date = parse_date("1985-04")?;
    assert_eq!("1985-04-01", &date.to_string());

    let date = parse_date("1985")?;
    assert_eq!("1985-01-01", &date.to_string());

    let date = parse_date("--0412")?;
    assert_eq!("0000-04-12", &date.to_string());

    let date = parse_date("---12")?;
    assert_eq!("0000-01-12", &date.to_string());

    // Trigger some branches
    assert!(parse_date("-").is_err());

    Ok(())
}

#[test]
fn types_date_time() -> Result<()> {
    let date_time = parse_date_time("20090808T1430-0500")?;
    assert_eq!("2009-08-08 14:30:00.0 -05:00:00", &date_time.to_string());

    let date_time = parse_date_time("19961022T140000Z")?;
    assert_eq!("1996-10-22 14:00:00.0 +00:00:00", &date_time.to_string());

    let date_time = parse_date_time("19961022T140000+0800")?;
    assert_eq!("1996-10-22 14:00:00.0 +08:00:00", &date_time.to_string());

    let date_time = parse_date_time("19961022T140000")?;
    assert_eq!("1996-10-22 14:00:00.0 +00:00:00", &date_time.to_string());

    let date_time = parse_date_time("--1022T1400")?;
    assert_eq!("0000-10-22 14:00:00.0 +00:00:00", &date_time.to_string());

    let date_time = parse_date_time("---22T14")?;
    assert_eq!("0000-01-22 14:00:00.0 +00:00:00", &date_time.to_string());

    Ok(())
}

#[test]
fn types_date_and_or_time() -> Result<()> {
    let value: DateAndOrTime = "19961022T140000".parse()?;
    if let DateAndOrTime::DateTime(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("1996-10-22 14:00:00.0 +00:00:00", &value.to_string());
    } else {
        panic!("expecting DateTime variant");
    }

    let value: DateAndOrTime = "--1022T1400".parse()?;
    if let DateAndOrTime::DateTime(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("0000-10-22 14:00:00.0 +00:00:00", &value.to_string());
    } else {
        panic!("expecting DateTime variant");
    }

    let value: DateAndOrTime = "---22T14".parse()?;
    if let DateAndOrTime::DateTime(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("0000-01-22 14:00:00.0 +00:00:00", &value.to_string());
    } else {
        panic!("expecting DateTime variant");
    }

    let value: DateAndOrTime = "19850412".parse()?;
    if let DateAndOrTime::Date(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("1985-04-12", &value.to_string());
    } else {
        panic!("expecting Date variant");
    }

    let value: DateAndOrTime = "1985-04".parse()?;
    if let DateAndOrTime::Date(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("1985-04-01", &value.to_string());
    } else {
        panic!("expecting Date variant");
    }

    let value: DateAndOrTime = "1985".parse()?;
    if let DateAndOrTime::Date(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("1985-01-01", &value.to_string());
    } else {
        panic!("expecting Date variant");
    }

    let value: DateAndOrTime = "--0412".parse()?;
    if let DateAndOrTime::Date(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("0000-04-12", &value.to_string());
    } else {
        panic!("expecting Date variant");
    }

    let value: DateAndOrTime = "---12".parse()?;
    if let DateAndOrTime::Date(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("0000-01-12", &value.to_string());
    } else {
        panic!("expecting Date variant");
    }

    let value: DateAndOrTime = "T102200".parse()?;
    if let DateAndOrTime::Time(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("10:22:00.0", &value.0.to_string());
    } else {
        panic!("expecting Time variant");
    }

    let value: DateAndOrTime = "T1022".parse()?;
    if let DateAndOrTime::Time(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("10:22:00.0", &value.0.to_string());
    } else {
        panic!("expecting Time variant");
    }

    let value: DateAndOrTime = "T10".parse()?;
    if let DateAndOrTime::Time(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("10:00:00.0", &value.0.to_string());
    } else {
        panic!("expecting Time variant");
    }

    let value: DateAndOrTime = "T-2200".parse()?;
    if let DateAndOrTime::Time(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("0:22:00.0", &value.0.to_string());
    } else {
        panic!("expecting Time variant");
    }

    let value: DateAndOrTime = "T--00".parse()?;
    if let DateAndOrTime::Time(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("0:00:00.0", &value.0.to_string());
    } else {
        panic!("expecting Time variant");
    }

    let value: DateAndOrTime = "T102200Z".parse()?;
    if let DateAndOrTime::Time(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("10:22:00.0", &value.0.to_string());
    } else {
        panic!("expecting Time variant");
    }

    let value: DateAndOrTime = "T102200-0800".parse()?;
    if let DateAndOrTime::Time(value) = value {
        //let value = value.get(0).unwrap();
        assert_eq!("10:22:00.0", &value.0.to_string());
        assert_eq!("-08:00:00", &value.1.to_string());
    } else {
        panic!("expecting Time variant");
    }

    Ok(())
}

#[test]
fn types_timestamp() -> Result<()> {
    let timestamp = parse_timestamp("19961022T140000")?;
    assert_eq!("1996-10-22 14:00:00.0 +00:00:00", &timestamp.to_string());

    let timestamp = parse_timestamp("19961022T140000Z")?;
    assert_eq!("1996-10-22 14:00:00.0 +00:00:00", &timestamp.to_string());

    let timestamp = parse_timestamp("19961022T140000-05")?;
    assert_eq!("1996-10-22 14:00:00.0 -05:00:00", &timestamp.to_string());

    let timestamp = parse_timestamp("19961022T140000-0500")?;
    assert_eq!("1996-10-22 14:00:00.0 -05:00:00", &timestamp.to_string());
    Ok(())
}

/*
#[test]
fn prop_time_offset() {
    let value = "020655-0057";
    let (time, offset) = parse_time(&value).unwrap();
    println!("{}", offset);
}
*/

proptest! {
    #[test]
    fn prop_parse_time_random(s in "\\PC*") {
        let _ = parse_time(&s);
    }

    #[test]
    fn prop_parse_time_all(s in "[0-9]{2}[0-9]{2}[0-9]{2}") {
        let _ = parse_time(&s);
    }

    #[test]
    fn prop_parse_time_valid_utc(h in 0u8..24, m in 0u8..60, s in 0u8..60) {
        // Without Z
        let (time, offset) = parse_time(
            &format!("{:02}{:02}{:02}", h, m, s)).unwrap();
        let (h2, m2, s2) = (time.hour(), time.minute(), time.second());
        prop_assert_eq!((h, m, s), (h2, m2, s2));
        assert_eq!(UtcOffset::UTC, offset);

        // With Z
        let (time, offset) = parse_time(
            &format!("{:02}{:02}{:02}Z", h, m, s)).unwrap();
        let (h2, m2, s2) = (time.hour(), time.minute(), time.second());
        prop_assert_eq!((h, m, s), (h2, m2, s2));
        assert_eq!(UtcOffset::UTC, offset);
    }

    #[test]
    fn prop_parse_time_valid_offset(
        h in 0u8..24,
        m in 0u8..60,
        s in 0u8..60,
        offset_h in 0u8..24,
        offset_m in 0u8..60) {

        let value = format!("{:02}{:02}{:02}-{:02}{:02}", h, m, s, offset_h, offset_m);

        // Negative to the West
        let (time, offset) = parse_time(
            &value).unwrap();
        let (h2, m2, s2) = (time.hour(), time.minute(), time.second());
        prop_assert_eq!((h, m, s), (h2, m2, s2));

        /*
        if offset_h > 0 || offset_m > 0 {
            assert!(offset.is_negative());
        }

        println!("({} - {}) : {}", offset_h, offset_m, offset);

        // Negative offsets yield negative i8 so we convert back to
        // u8 for comparison
        let (offset_hours, offset_minutes,_) = offset.as_hms();
        let abs_offset_hours = (offset_hours + 127) as u8;
        //let abs_offset_minutes = (offset_minutes + 127) as u8;

        println!("{} {}", abs_offset_hours, offset_minutes);
        */
    }

    #[test]
    fn prop_date_random(s in "\\PC*") {
        let _ = parse_date(&s);
    }

    #[test]
    fn prop_parse_date_all(s in "[0-9]{4}[0-9]{2}[0-9]{2}") {
        let _ = parse_date(&s);
    }

    #[test]
    fn prop_parse_date_valid(
        y in 0i32..10000,
        m in 1u8..=12,
        // Only test upto 29 otherwise an error in February
        d in 1u8..29) {
        let value = format!("{:04}{:02}{:02}", y, m, d);
        //println!("{}", value);
        let date = parse_date(&value).unwrap();

        let m2: u8 = date.month().try_into().unwrap();
        let (y2, d2) = (date.year(), date.day());
        prop_assert_eq!((y, m, d), (y2, m2, d2));
    }

    #[test]
    fn prop_date_time_random(s in "\\PC*") {
        let _ = parse_date_time(&s);
    }

    #[test]
    fn prop_parse_date_time_all(
        s in "[0-9]{4}[0-9]{2}[0-9]{2}T[0-9]{2}[0-9]{2}[0-9]{2}[+-][0-9]{2}[0-9]{2}") {
        let _ = parse_date_time(&s);
    }

    #[test]
    fn prop_parse_date_time_valid(
        y in 0i32..10000,
        m in 1u8..=12,
        // Only test upto 29 otherwise an error in February
        d in 1u8..29,
        h in 0u8..24,
        mi in 0u8..60,
        s in 0u8..60,
        offset_h in 0u8..24,
        offset_m in 0u8..60,
    ) {
        // No offset
        let value = format!(
            "{:04}{:02}{:02}T{:02}{:02}{:02}",
            y, m, d, h, mi, s);
        let date_time = parse_date_time(&value).unwrap();
        let m2: u8 = date_time.month().try_into().unwrap();
        let (y2, d2) = (date_time.year(), date_time.day());
        let (h2, mi2, s2) = (
            date_time.hour(), date_time.minute(), date_time.second());
        prop_assert_eq!((y, m, d, h, mi, s), (y2, m2, d2, h2, mi2, s2));

        // TODO: test negative offset
        // TODO: test positive offset
    }
}
