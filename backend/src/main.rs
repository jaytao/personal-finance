use chrono::NaiveDate;
use csv::StringRecord;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

mod transaction;
use transaction::{Source, Transaction};
mod csv_parsers;

#[macro_use] extern crate rocket;
use rocket::serde::{json::Json};
use rocket::State;

struct AppState {
    transactions: Vec<Transaction>
}
impl AppState {
    fn new(transactions: Vec<Transaction>) -> AppState {
        AppState {
            transactions: transactions
        }
    }
}
#[get("/transactions")]
fn index(app_state: &State<AppState>) -> Json<Vec<Transaction>>{
    Json(app_state.transactions.clone())
}

#[launch]
fn rocket() -> _ {
    let transactions = read_statements();
    rocket::build().mount("/", routes![index]).manage(AppState::new(transactions))
}

fn read_statements() -> Vec<Transaction>{
    let root_dir = "statements";
    let mut transactions: Vec<Transaction> = vec![];
    let mut per_source: HashMap<Source, f32> = HashMap::new();

    for entry in fs::read_dir(root_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let _file_name = entry.file_name();
        let source_folder = _file_name.to_str().unwrap();
        //println!("{} {}", source_folder, entry.path().to_str().unwrap());
        let parser = match source_folder {
            "bofa" => csv_parsers::parse_record_bofa,
            "chase" => csv_parsers::parse_record_chase,
            "chase-bank" => csv_parsers::parse_record_chase_bank,
            "amex" => csv_parsers::parse_record_amex,
            "venmo" => csv_parsers::parse_record_venmo,
            "bilt" => csv_parsers::parse_record_bilt,
            _ => panic!("Found unknown source"),
        };
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }
        for csv_file in fs::read_dir(entry.path().to_str().unwrap()).unwrap() {
            let csv_entry = csv_file.unwrap();
            transactions.append(&mut parse_csv(
                &csv_entry.path().to_str().unwrap().to_string(),
                parser,
            ));
        }
    }
    transactions.sort_by(|a, b| a.date.cmp(&b.date));
    for transaction in &transactions {
        *per_source.entry(transaction.source).or_insert(0.0) += transaction.amount;
        //if transaction.source == Source::BofA {
        //    println!("{:?}", transaction.amount);
        //}
    }
    for (key, value) in per_source.iter() {
        println!("{:?}:\t\t{}", key, value);
    }
    transactions
}

fn parse_csv(path: &String, parser: fn(&StringRecord) -> Transaction) -> Vec<Transaction> {
    let mut file = fs::File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(buf_reader);
    let mut contents = String::new();
    rdr.records()
        .map({
            |result| match &result {
                Ok(record) => Ok(parser(record)),
                Err(err) => {
                    println!("Err: {} {:?}: {}", path, result, &err);
                    Err("")
                }
            }
        })
        .filter_map(|result| match result {
            Ok(x) => Some(x),
            Err(_) => None,
        })
        .collect()
}
