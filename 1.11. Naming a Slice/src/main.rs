use std::str::FromStr;

fn main() -> Result<(),std::num::ParseFloatError> {
    //            0123456789012345678901234567890123456789012345678901234567890
    let record = "....................100          .......513.25     ..........".as_bytes();
    let marker = "                    xxxxxxxxxxxxx       yyyyyyyyyyy          ".as_bytes();

    let x = f64::from_str(&String::from_utf8_lossy(&record[marker.iter().position(|c|c==&b'x').unwrap() 
                                                           ..= 
                                                           marker.iter().rposition(|c|c==&b'x').unwrap()]).trim())?;

    let y = f64::from_str(&String::from_utf8_lossy(&record[marker.iter().position(|c|c==&b'y').unwrap() 
                                                           ..= 
                                                           marker.iter().rposition(|c|c==&b'y').unwrap()]).trim())?;

    println!("{} {}",x, y);
    Ok(())
}
