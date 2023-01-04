pub fn _let_me_borrow() {
    let mut _book = String::from("Harry Potter ");
    _book_name(&mut _book);

    println!("Let me borrow your book {_book}");
}

pub fn _borrow_and_clone() {
    let mut _money = String::from("Dollar");
    let _borrowed_money = &_money;
    let _counterfit_money = &_money.clone();

    println!("My money is {} and this is another money {} and this counterfit money {}", _money, _borrowed_money, _counterfit_money);
}

fn _book_name(title: &mut String) {
    title.push_str("and Philosopher Stone");
}

pub fn _calculated_length_string() {
    let s1 = String::from("Harry Potter and the Half-blood Prince");

    let len = _length(&s1);
    println!("Book: {}\nLength of String: {}", s1, len);
}

fn _length(s: &String) -> usize {
    return s.len();
}

fn _first_word(s: &String) -> usize {
    let byte = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

pub fn _all_basic_borrow_and_refrences() {
    let some_word = String::from("Manga");
    let first_word_of_string = _first_word(&some_word);

    println!("{} {}", some_word, first_word_of_string);
}