pub fn raindrops(n: u32) -> String {
    let mut song: String = String::new();

    if n % 3 == 0 {
        song.push_str("Pling");
    }

    if n % 5 == 0 {
        song.push_str("Plang");
    }

    if n % 7 == 0 {
        song.push_str("Plong");
    }

    // if n does not have 3,5 or 7 as factors
    if song.is_empty() {
        song = n.to_string();
    }

    song
}
