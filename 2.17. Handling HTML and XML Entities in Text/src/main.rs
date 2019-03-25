use htmlescape::{encode_minimal, decode_html};

fn encode_ascii(ins :&str) -> String {
    ins.chars().map(|ch| 
                   if ch.is_ascii() { 
                       ch.to_string() 
                   } else {
                       format!("&#{};", ch as u32)
                   }).collect::<Vec<_>>().join("")
}

fn main() {

    println!("{}", encode_minimal(r#"Elements are written as "<tag>text</tag>"."#));
    println!("{}", encode_ascii("picy Jalapeño"));

    // note that htmlescape is not tolerant as Python's html.parser 
    println!("{:?}", decode_html("Spicy &quot;Jalape&#241;o&quot;."));
}
