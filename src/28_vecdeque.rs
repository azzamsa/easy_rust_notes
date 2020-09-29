// A VecDeque is a Vec that is good at popping items both off the front and the back.

use std::collections::VecDeque;

fn main() {
    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for i in 0..600000 {
        my_vec.pop_front(); // pop_front is like .pop but for the front
        //println!("{:?}", my_vec);
    }
}

// NOTE:
//.pop() remove without moving other item
// .remove() remove and move other item to the right
// There are also other method that you can use: .pop_back(), .push_front()
