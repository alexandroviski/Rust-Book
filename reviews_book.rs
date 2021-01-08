use std::collections::HashMap;

fn main() {
    let mut book_reviews = HashMap::new();

    // review some books.
    book_reviews.insert(
        "adventures of huckleberry finn".to_string(),
        "my favorite book".to_string(),
    );

    book_reviews.insert("grimm's fairy tales".to_string(), "masterpiece".to_string());

    book_reviews.insert(
        "pride and prejudice".to_string(),
        "very enjoyable".to_string(),
    );

    book_reviews.insert(
        "the adventures of sherlock holmes".to_string(),
        "eye lyked it alot".to_string(),
    );

    // check for a especific one
    // when collections store owned values (string), they can still be
    // queried using references (&str)
    if !book_reviews.contains_key("les miserables") {
        println!(
            "we've got {} reviews, but les miserables ain't one.",
            book_reviews.len()
        );
    }
    // oops,this review have a lot of spellings mistakes,lets delete it
    book_reviews.remove("the adventures of sherlock holmes");

    // look up the values asssociated with some key
    let to_find = ["pride and prejudice", "alice's adventures in wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed", book),
        }
    }

    // look up the value for key (will panic if the key is not found)
    println!("review for Jane: {}", book_reviews["pride and prejudice"]);

    // iterate over overything
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }
}
