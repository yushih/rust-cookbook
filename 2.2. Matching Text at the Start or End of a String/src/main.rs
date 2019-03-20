fn main() {
    let filenames = [ "Makefile", "foo.c", "bar.py", "spam.c", "spam.h" ];

    println!("{:?}", filenames.iter().filter(|filename|[".c", ".h"].iter().any(|sufix|filename.ends_with(sufix))).collect::<Vec<_>>());
}
