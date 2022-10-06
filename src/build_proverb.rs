// rank - easy
// source - exercism
pub fn run(list: &[&str]) -> String {

    if list.len() == 0 {
        return String::new();
    }

    let mut res:Vec<String> = Vec::new();
    let mut i = 0;

    if list.len() == 1{
    res.push(format!("And all for the want of a {first}.", first = list[0]));
    }

    while i < list.len() - 1 {
        res.push(format!("For want of a {first} the {second} was lost.", first = list[i], second = list[i + 1]));
        // println!("{}",format!("For want of a {first} the {second} was lost", first = list[i], second = list[i + 1]));
        if i == list.len() - 2 {
            res.push(format!("And all for the want of a {first}.", first = list[0]));
        }
        i += 1;
    }
    res.join("\n")
}

#[test]
fn test_two_pieces() {
    let input = vec!["nail", "shoe"];
    let expected = vec![
        "For want of a nail the shoe was lost.",
        "And all for the want of a nail.",
    ]
        .join("\n");
    assert_eq!(run(&input), expected);
}
// Notice the change in the last line at three pieces.
#[test]
#[ignore]
fn test_three_pieces() {
    let input = vec!["nail", "shoe", "horse"];
    let expected = vec![
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "And all for the want of a nail.",
    ]
        .join("\n");
    assert_eq!(run(&input), expected);
}
#[test]
#[ignore]
fn test_one_piece() {
    let input = vec!["nail"];
    let expected = String::from("And all for the want of a nail.");
    assert_eq!(run(&input), expected);
}
#[test]
#[ignore]
fn test_zero_pieces() {
    let input: Vec<&str> = vec![];
    let expected = String::new();
    assert_eq!(run(&input), expected);
}
#[test]
#[ignore]
fn test_full() {
    let input = vec![
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];
    let expected = vec![
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "For want of a horse the rider was lost.",
        "For want of a rider the message was lost.",
        "For want of a message the battle was lost.",
        "For want of a battle the kingdom was lost.",
        "And all for the want of a nail.",
    ]
        .join("\n");
    assert_eq!(run(&input), expected);
}
#[test]
#[ignore]
fn test_three_pieces_modernized() {
    let input = vec!["pin", "gun", "soldier", "battle"];
    let expected = vec![
        "For want of a pin the gun was lost.",
        "For want of a gun the soldier was lost.",
        "For want of a soldier the battle was lost.",
        "And all for the want of a pin.",
    ]
        .join("\n");
    assert_eq!(run(&input), expected);
}
