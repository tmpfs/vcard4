use anyhow::Result;
use vcard_compact::types::*;

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

    Ok(())
}

#[test]
fn types_date_time() -> Result<()> {
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
    let timestamp= parse_timestamp("19961022T140000")?;
    assert_eq!("1996-10-22 14:00:00.0 +00:00:00", &timestamp.to_string());

    let timestamp= parse_timestamp("19961022T140000Z")?;
    assert_eq!("1996-10-22 14:00:00.0 +00:00:00", &timestamp.to_string());

    let timestamp= parse_timestamp("19961022T140000-05")?;
    assert_eq!("1996-10-22 14:00:00.0 -05:00:00", &timestamp.to_string());

    let timestamp= parse_timestamp("19961022T140000-0500")?;
    assert_eq!("1996-10-22 14:00:00.0 -05:00:00", &timestamp.to_string());
    Ok(())
}
