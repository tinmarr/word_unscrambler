// Copyright (C) 2023 Martin Chaperot
//
// This file is part of word_unscrambler.
//
// word_unscrambler is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// word_unscrambler is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with word_unscrambler.  If not, see <http://www.gnu.org/licenses/>.
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};

const LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn load_dictionary() -> BTreeMap<u128, Vec<String>> {
    let data = fs::read_to_string("assets/DL.txt").expect("Unable to read dictionary file");
    let mut map: BTreeMap<u128, Vec<String>> = BTreeMap::new();
    for line in data.lines() {
        let word = line.to_string().to_lowercase();
        if word == "" {
            continue;
        }
        let i = word_2_int(&word);
        match map.get_mut(&i) {
            Some(vector) => {
                vector.push(word);
            }
            None => {
                map.insert(i, vec![word]);
            }
        }
    }
    map
}

fn word_2_int(word: &String) -> u128 {
    let mut word_int: u128 = 0;
    for letter in word.chars() {
        let i: u32 = match LETTERS.binary_search(&letter) {
            Ok(i) => i,
            Err(_) => continue,
        }
        .try_into()
        .expect("If this panics something went horribly wrong");
        word_int += 2u128.pow(4u32 * i);
    }
    word_int
}

fn main() {
    let map = load_dictionary();
    loop {
        let mut word = String::new();

        print!("Enter a scrambled word: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        word = word.trim().to_string().to_lowercase();
        match map.get(&word_2_int(&word)) {
            Some(vector) => {
                println!("{:?}", vector);
            }
            None => {
                println!("No match found");
            }
        }

        print!("Try again? [Y/n]: ");
        io::stdout().flush().unwrap();

        let mut again = String::new();

        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read line");

        if again.trim().to_lowercase() != "n" {
            continue;
        } else {
            break;
        }
    }
}
