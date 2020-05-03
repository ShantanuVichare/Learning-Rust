

struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let max_val = 5;
        if self.count < max_val {
            self.count += 1;
            Some(self.count)
        }
        else {
            None
        }
    }
}

pub fn run() {
    // Iterators implement the Iterator Trait:
    // pub trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }
    // Item is an Associated type of the Iterator Trait

    // All Iterators are lazy i.e. They do not execute/produce any result unless "consumed"

    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter(); // .iter() will give Immutable references to the values
    
    // Each .next() call consumes(modifies permanently) the iterator.. thus it needs to be Mutable
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // .iter_mut() will give Mutable references to the values
    // .into_iter() will give Ownership of the values

    // Iterator Trait implements various other methods or.. "Adaptors"!
    // "Consuming Adaptors" consume the iterator by internally call the .next() method
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // .sum() consumes the iterator by taking it's ownership.. Hence, 'v1_iter' is not valid now
    assert_eq!(total, 6);

    // "Iterator Adaptors" change iterators to different kinds of iterators
    let v1_plus_one_iter = v1.iter().map(|x| x+1);
    // .collect is another Consuming adapter which collects the values into a Collection data type
    let v1_plus_one: Vec<_> = v1_plus_one_iter.collect();

    // .filter takes a Closure which returns a bool
    // .filter is also an Iterator Adaptor and thus returns an Iterator
    let evens:Vec<_> = v1_plus_one.into_iter().filter(|s| s%2==0).collect();
    assert_eq!(evens, vec![2,4]);


    // Implementing Iterators using Iterator Trait
    // Implementing only Item and .next() method
    let c0 = Counter::new();
    let c1 = Counter::new().skip(1);
    let zip_iter = c0.zip(c1); // .zip will call Into Iterator implicitly and hence move both 'c0' and 'c1'
    // Zip Iterator will be of shorter length of the two Iterators - Returns None when either of the Iterators return None
    
    let sum = zip_iter.map(|(a,b)| a*b).filter(|x| x%3 == 0).sum::<usize>(); 
    // Since .sum() requires type annotations, it can be annotated with the result or passed to the function as done above
    println!("Sum of multiples divisible by 3: {}", sum);
}