// panic: to remind you forbidden things in code


fn prints_three_things(vector: Vec<i32>) {
     if vector.len() != 3 {
        panic!("my_vec must always have three items") // will panic if the length is not 3
    }
  println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn main() {
  let my_vec = vec![8, 9, 10]; // Now my_vec has six things
  prints_three_things(my_vec);

    let my_vec1 = vec![8, 9, 10, 10, 55, 99]; // Now my_vec has six things
  prints_three_things(my_vec1);
}

// NOTE:
// other things you can use are: assert, assert_eq, assert_ne
