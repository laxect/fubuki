fn main() -> Result<(), ()> {
    index_gen::read_files().expect("error!");
    Ok(())
}
