use english_numbers::convert_all_fmt;

fn main() {
    println!("This program will print the lyrics of the The Twelve Days of Christmas: \n");

    let day_string: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];

    let items_of_the_day: [&str; 12] = ["partridge in a pear tree",
        "turtle doves and",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"];

    for day_of_christmas in 1..13 {
        println!("On the {} day of Christmas, my true love sent to me",
            day_string[day_of_christmas -1]);

        let mut item_day = day_of_christmas;

        while item_day > 0{
            let day_prefix = &convert_all_fmt(item_day as i64);

            println!("{} {}", 
                if item_day == 1 { "A" } else {day_prefix},
                items_of_the_day[item_day - 1]);
            item_day -= 1;
        }
        println!("\n");
    }
}

/* Lyrics to The Twelve Days of Christmas 
* (source https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics):
* [Verse 1]
* On the first day of Christmas, my true love sent to me
* A partridge in a pear tree
* 
* [Verse 2]
* On the second day of Christmas, my true love sent to me
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 3]
* On the third day of Christmas, my true love sent to me
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 4]
* On the fourth day of Christmas, my true love sent to me
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 5]
* On the fifth day of Christmas, my true love sent to me
* Five golden rings
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 6]
* On the sixth day of Christmas, my true love sent to me
* Six geese a-laying
* Five golden rings
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 7]
* On the seventh day of Christmas, my true love sent to me
* Seven swans a-swimming
* Six geese a-laying
* Five golden rings
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 8]
* On the eighth day of Christmas, my true love sent to me
* Eight maids a-milking
* Seven swans a-swimming
* Six geese a-laying
* Five golden rings
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 9]
* On the ninth day of Christmas, my true love sent to me
* Nine ladies dancing
* Eight maids a-milking
* Seven swans a-swimming
* Six geese a-laying
* Five golden rings
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 10]
* On the tenth day of Christmas, my true love sent to me
* Ten lords a-leaping
* Nine ladies dancing
* Eight maids a-milking
* Seven swans a-swimming
* Six geese a-laying
* Five golden rings
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 11]
* On the eleventh day of Christmas, my true love sent to me
* Eleven pipers piping
* Ten lords a-leaping
* Nine ladies dancing
* Eight maids a-milking
* Seven swans a-swimming
* Six geese a-laying
* Five golden rings
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
* 
* [Verse 12]
* On the twelfth day of Christmas, my true love sent to me
* Twelve drummers drumming
* Eleven pipers piping
* Ten lords a-leaping
* Nine ladies dancing
* Eight maids a-milking
* Seven swans a-swimming
* Six geese a-laying
* Five golden rings
* Four calling birds
* Three french hens
* Two turtle doves and
* A partridge in a pear tree
*/
