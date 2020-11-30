const VERSES: [(&str, &str); 12] = [
    ("first", "A partridge in a pear tree"),
    ("second", "Two turtle doves, and"),
    ("third", "Three French hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Five golden rings"),
    ("sixth", "Six geese a-laying"),
    ("seventh", "Seven swans a-swimming"),
    ("eighth", "Eight maids a-milking"),
    ("ninth", "Nine ladies dancing"),
    ("10th", "Ten lords a-leaping"),
    ("11th", "Eleven pipers piping"),
    ("12th", "Twelve drummers drumming"),
];

fn main() {
    for i in 0..VERSES.len() {
        println!("On the {} day of Christmas my true love sent to me", VERSES[i].0);
        for j in (0..i+1).rev() {
            println!("{}", VERSES[j].1);
        }
        println!("");
    }
}
