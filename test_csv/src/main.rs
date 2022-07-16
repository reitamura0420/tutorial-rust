extern crate csv;
extern crate serde_derive;

use csv::{Error, Reader, ReaderBuilder, Writer};
use serde::de::{self, Deserialize};
use serde::Deserializer;
use serde_derive::{Deserialize, Serialize};
use std::{io, str::FromStr};

fn main() {
    // simple_csv();
    // serde_csv();
    // serde_csv_2();
    // filter_csv();
    // writer_csv();
    // serialize_csv();
    transform_csv();
}

fn simple_csv() {
    let csv = "year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        // StringRecord: 有効なUTF-8バイトとして格納された1つのCSVレコード。
        // 有効なUTF-8でないCSVデータを読み込んだ場合、CSVリーダは無効なUTF-8エラーを返します。
        // UTF-8が無効である可能性のあるデータを読み込む必要がある場合は、UTF-8を仮定しないByteRecordを使用することをお勧めします。
        let record = record.unwrap();
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        );
    }
}

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn serde_csv() {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record.unwrap();
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year, record.make, record.model, record.description
        );
    }
}

#[derive(Debug, Deserialize)]
struct Record_2 {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")] // 無効なデータをNoneにしてくれる
    id: Option<u64>,
}

fn serde_csv_2() {
    let data = "name\tplace\tid
        Mark\tMelbourne\t46
        Ashley\tZurich\t92";

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t') // delimiterで区切り文字の指定をする
        .from_reader(data.as_bytes());
    for result in reader.deserialize::<Record_2>() {
        println!("{:?}", result.unwrap());
    }
}

fn filter_csv() {
    let query = "CA";
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";

    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers().unwrap()).unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record).unwrap();
        }
    }

    wtr.flush().unwrap();
}

fn writer_csv() {
    let mut wtr = csv::Writer::from_writer(io::stdout()); // CSVレコードへの自動シリアライズしてくれる

    wtr.write_record(&["Name", "Place", "ID"]).unwrap(); // 文字列のみの場合はwrite_recordを使う

    wtr.serialize(("Mark", "Sydney", 87)).unwrap(); // 数値やoptionを使う場合はserializeを使う
    wtr.serialize(("Ashley", "Dublin", 32)).unwrap();
    wtr.serialize(("Akshat", "Delhi", 11)).unwrap();

    wtr.flush().unwrap();
}

#[derive(Serialize)]
struct Record_3<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}
fn serialize_csv() {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = Record_3 {
        // カスタム構造体はSerdeのシリアライズを使う
        name: "Mark",
        place: "Melbourne",
        id: 56,
    };
    let rec2 = Record_3 {
        name: "Ashley",
        place: "Sydney",
        id: 64,
    };
    let rec3 = Record_3 {
        name: "Akshat",
        place: "Delhi",
        id: 98,
    };

    wtr.serialize(rec1).unwrap();
    wtr.serialize(rec2).unwrap();
    wtr.serialize(rec3).unwrap();

    wtr.flush().unwrap();
}

#[derive(Debug)]
struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Deserialize)]
struct Row {
    color_name: String,
    color: HexColor,
}

impl FromStr for HexColor {
    type Err = Error;

    // 変換処理があるならfrom_strで書く
    fn from_str(hex_color: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = hex_color.trim_matches('#');
        Ok(HexColor {
            red: u8::from_str_radix(&trimmed[..2], 16).unwrap(),
            green: u8::from_str_radix(&trimmed[2..4], 16).unwrap(),
            blue: u8::from_str_radix(&trimmed[4..6], 16).unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

fn transform_csv() {
    let data = "color_name,color
red,#ff0000
green,#00ff00
blue,#0000FF
periwinkle,#ccccff
magenta,#ff00ff"
        .to_owned();
    let mut out = Writer::from_writer(vec![]); // output用データの格納場所
    let mut reader = Reader::from_reader(data.as_bytes()); // CSV読み込み
    for result in reader.deserialize::<Row>() {
        // CSVをdeserializeするよ
        let res = result.unwrap();
        out.serialize((
            // 書き込み
            res.color_name,
            res.color.red,
            res.color.green,
            res.color.blue,
        ))
        .unwrap();
    }
    let written = String::from_utf8(out.into_inner().unwrap()).unwrap(); // into_inner()で中身を持ってこれる。ただbyte型なのでfrom_utf8しないと読めない。
    assert_eq!(Some("magenta,255,0,255"), written.lines().last());
    println!("{}", written);
}
