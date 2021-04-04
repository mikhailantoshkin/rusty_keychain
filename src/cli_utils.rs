pub fn show_choise(choise: &str) -> bool {
    loop {
        println!("{} Y/n", choise);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read a line");
        match input.to_lowercase().trim() {
            "y" => return true,
            "n" => return false,
            _ => {
                print!("Please enter Y or n. ");
                continue;
            }
        }
    }
}
