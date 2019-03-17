use itertools::Itertools;

struct Row {
    address: String,
    date: String
}

fn main() {
    let mut rows = vec![
        Row { address: "5412 N CLARK".to_string(), date: "07/01/2012".to_string()},    
        Row { address: "5148 N CLARK".to_string(), date: "07/04/2012".to_string()},    
        Row { address: "5800 E 58TH".to_string(), date: "07/02/2012".to_string()},     
        Row { address: "2122 N CLARK".to_string(), date: "07/03/2012".to_string()},    
        Row { address: "5645 N RAVENSWOOD".to_string(), date: "07/02/2012".to_string()},
        Row { address: "1060 W ADDISON".to_string(), date: "07/02/2012".to_string()},  
        Row { address: "4801 N BROADWAY".to_string(), date: "07/01/2012".to_string()}, 
        Row { address: "1039 W GRANVILLE".to_string(), date: "07/04/2012".to_string()}];

    rows.sort_by_key(|Row {date, ..}|date.clone());

    for (date, rows) in &rows.iter().group_by(|Row {address:_, date}|date) {
        println!("{}", date);
        for Row {address, ..} in rows {
            println!("\t{}", address);
        }
    }
}
