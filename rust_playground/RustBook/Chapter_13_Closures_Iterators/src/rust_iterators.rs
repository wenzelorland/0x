// Iterators allow you to iterate over a sequence of elements
// Iterators encapsulate the logic for iterating over data structures in a uniformed way
// Iterators are part of the Rust standard library

use std::vec;
mod own_iterator;

// The Iterator trait is implemented as following
pub trait Iterator {
    type Item;

    // this is the only method that one needs to implement
    // this needs to be mutable reference as we need to track at what item we are
    fn next(&mut self) -> Option<Self::Item>;

}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// taking ownership of the inserted Shoe type variable 
fn shoes_in_my_size(shoes:Vec<Shoe>, shoe_size:u32) -> Vec<Shoe> {
    // filter create also an iterator; i.e. if the closure returns true for the element, then this element will be returned
    // example how closure capture their environment
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
}

#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    // sum will repeatedly call the next method to get the next element in the sequence and add those elements up
    let total : i32 = v1_iter.sum();

    assert_eq!(total, 6);
}


pub fn run() {
    println!("Hello to Rust Iterators!");

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter(); // iterators are lazy, i.e. they are only constructed when used

    for value in v1_iter {
        println!("Got: {}", value);
    };
    // this is a consumer method on the iterator -> in order to retrieve the result we need to call the collect mehtod on it
    // since iterators are lazy in rust and we need to "invoke" the result 
    // this creates an iterator over which we call a closure  and then we collect the result in which it transforms the iterator into a collection
    let v2: Vec<_> = v1.iter().map(|x| x +1).collect();

    assert_eq!(v2, vec![2,3,4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 11,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size :10,
                    style: String::from("sneaker"),
                }
            ]
        )


    }

    #[test]
    fn calling_next_directly() {
        let mut counter = own_iterator::Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    } 

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = own_iterator::Counter::new()
            // the zip method takes 2 iterators and zips them up in one iterator by creating pairs
            // the first iterator will be the one zip is called on, while the second iterator is the one passed to the method
            .zip(own_iterator::Counter::new().skip(1)) // skip method will create an iterator that skips the first n elements, here the first element
            .map(|(a, b)| a*b) // map takes a closure which it will apply to each element of the iterator it is called on
            .filter(|x| x%3 == 0) // filter will only return the elements in the iterator where the closure expression is true
            .sum(); // this consumes the iterator as it represents one of the consumer methods on iterators
        assert_eq!(18, sum);

    }

}