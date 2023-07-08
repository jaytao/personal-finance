use crate::transaction::{Source, Transaction};
use chrono::NaiveDate;
use csv::StringRecord;

pub fn parse_record_chase(record: &StringRecord) -> Transaction {
    let parse_from_str = NaiveDate::parse_from_str;
    let amount = str::parse(&record.get(5).unwrap().replace(",", "")).unwrap_or_else(|f| {
        println!("Errored out on {}. {:?}", f, record);
        0.0
    });
    Transaction::new(
        Source::Chase,
        parse_from_str(record.get(0).unwrap(), "%m/%d/%Y").unwrap(),
        amount,
        record.get(2).unwrap().to_string(),
    )
}

pub fn parse_record_chase_bank(record: &StringRecord) -> Transaction {
    let parse_from_str = NaiveDate::parse_from_str;
    let amount = str::parse(&record.get(3).unwrap().replace(",", "")).unwrap_or_else(|f| {
        println!("Errored out on {}. {:?}", f,record);
        0.0
    });
    Transaction::new(
        Source::ChaseBank,
        parse_from_str(record.get(1).unwrap(), "%m/%d/%Y").unwrap(),
        amount,
        record.get(2).unwrap().to_string(),
    )
}

pub fn parse_record_bofa(record: &StringRecord) -> Transaction {
    let parse_from_str = NaiveDate::parse_from_str;
    let amount = str::parse(&record.get(2).unwrap().replace(",", "")).unwrap_or_else(|f| {
        println!("Errored out on {}. {:?}", f,record);
        0.0
    });
    Transaction::new(
        Source::BofA,
        parse_from_str(record.get(0).unwrap(), "%m/%d/%Y").unwrap(),
        amount,
        record.get(1).unwrap().to_string(),
    )
}

pub fn parse_record_amex(record: &StringRecord) -> Transaction {
    let parse_from_str = NaiveDate::parse_from_str;
    let amount = -str::parse(&record.get(2).unwrap().replace(",", "")).unwrap_or_else(|f| {
        println!("Errored out on {}. {:?}", f,record);
        0.0
    });
    Transaction::new(
        Source::Amex,
        parse_from_str(record.get(0).unwrap(), "%m/%d/%Y").unwrap(),
        amount,
        record.get(1).unwrap().to_string(),
    )
}


pub fn parse_record_venmo(record: &StringRecord) -> Transaction {
    let parse_from_str = NaiveDate::parse_from_str;
    let amount_split: Vec<&str> = record.get(8).unwrap().split(" ").collect();
    let operator = match amount_split[0] {
        "+" => 1.0,
        "-" => -1.0,
        _ => panic!("Found something wrong: {:?}", record)
    };

    let amount = &amount_split[1][1..].replace(",", "");
    let amount_f = str::parse(amount).unwrap_or_else(|_| 0.0) * operator;
    Transaction::new(
        Source::Venmo,
        parse_from_str(record.get(2).unwrap(), "%Y-%m-%dT%H:%M:%S").unwrap(),
        amount_f,
        record.get(5).unwrap().to_string(),
    )
}