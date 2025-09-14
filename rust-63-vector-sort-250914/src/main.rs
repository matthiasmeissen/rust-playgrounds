use std::collections::HashMap;

fn main() {
    let mut vec  = vec![20, 40, 80, 42, 68, 200, 30, 62, 120, 62];
    println!("Input:    {:?}", vec);
    vec.sort();
    println!("Sorted:   {:?}", vec);

    if vec.len() % 2 == 0 {
        println!("Its even. {} and {}", vec[vec.len() / 2], vec[vec.len() / 2 - 1]);
    } else {
       println!("Its odd. {}", vec[vec.len() / 2]);
    }


    let mut map: HashMap<i32, usize> = HashMap::new();
    for val in vec {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}