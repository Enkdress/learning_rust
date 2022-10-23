// 1. Get the data from the csv
// 2. Create the struc and its constructor for the {DataRecord}
use csv::{ReaderBuilder, StringRecord, StringRecordsIter};
use std::{collections::HashMap, fs};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct HistoryRecord {
    rec_type: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<HistoryRecord>,
}

impl HistoryRecord {
    fn new(row: StringRecord) -> HistoryRecord {
        let life = row.get(3).unwrap().trim();
        let life: i32 = life.parse().unwrap_or(0);

        return HistoryRecord {
            rec_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life: life,
            options: vec![],
        };
    }
}

fn main() {
    let life: i32 = 100;
    let mut current_tag = FIRST_TAG;

    // Getting the csv string
    let content = fs::read_to_string(FILENAME).unwrap();

    // Transform that string into a csv reader
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    let history_data: HashMap<String, HistoryRecord> = build_history_tree(rdr.records());

    // Game loop
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Tu vida es: {}", life);

        if let Some(current_record) = history_data.get(current_tag) {
            println!("{}", current_record.text);

            if validate_player_life(life, current_record.life) == -1 {
                break;
            }

            // Iter through the options
            for (index, option) in current_record.options.iter().enumerate() {
                println!("[{}] {}", index, option.text);
            }

            // handle user selection
            let mut user_selection = String::new();
            std::io::stdin().read_line(&mut user_selection).unwrap();
            let user_selection = user_selection.trim().parse().unwrap_or(99);

            // update the current tag
            if let Some(selected_option) = &current_record.options.get(user_selection) {
                current_tag = &selected_option.tag;
            } else {
                println!("Ohhh sorry. Bad option");
            }
        } else {
            break;
        }
    }
}

fn build_history_tree(records: StringRecordsIter<&[u8]>) -> HashMap<String, HistoryRecord> {
    let mut history_data: HashMap<String, HistoryRecord> = HashMap::new();
    let mut last_record: String = "".to_string();

    // Create Main sentenses and options tree
    for result in records {
        let row = result.unwrap();
        let record = HistoryRecord::new(row);

        if record.rec_type == "SITUACION" {
            let record_tag = record.tag.clone();
            history_data.insert(record_tag.clone(), record);
            last_record = record_tag;
        } else if record.rec_type == "OPCION" {
            if let Some(situation_data) = history_data.get_mut(&last_record) {
                (*situation_data).options.push(record);
            }
        }
    }

    return history_data;
}

fn validate_player_life(mut current_life: i32, record_life: i32) -> i8 {
    let mut response = 0;
    current_life += record_life;
    if current_life <= 0 {
        response = -1;
        println!("\n==================\nGAME OVER\n==================\n");
    }

    return response;
}
