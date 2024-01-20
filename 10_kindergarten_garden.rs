// list of children's
const CHILDREN: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {

    let position = CHILDREN.iter().position(|&name| name == student).unwrap() * 2;

    let apply_char_to_position = |c:char| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _   => ""
    };

    diagram.lines().flat_map(|line|
        {
        line[position..=position+1].chars().map(apply_char_to_position)
    })
    .collect()
    
}
