// mod unique_email;
// mod reverse_string;
// mod armstrong_numbers;
// mod beer_song;
// mod prime_factors;
// mod differences_of_squares;
mod leap_year;

fn main() {
    // println!("{:?}", unique_email::run(vec!["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]));

    println!("{:?}", leap_year::run(2000));
    println!("{:?}", leap_year::run(1996));
    println!("{:?}", leap_year::run(1997));
    println!("{:?}", leap_year::run(1900));
}



