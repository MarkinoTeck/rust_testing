fn main() {

    // arr, stack allocated, can't change size
    let a = [1, 2, 3];

    // vector, heap allocated
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // creating a vect
    {
        let v2 = vec![1, 2, 3];
    }
    // v2 gets dropped as it's out of scope

    // getting an element from vect
    // can crash if array goes out of bound
    {
        let v = vec![1, 2, 3];

        let second = &v[1];
        println!("second element is: {}", second);
    }

    // use get element for out of bound safety
    {
        let mut v = vec![0, 1, 2, 3];

        let tenth_value = match v.get(10) {
            Some(int) => int,
            None => &0,
        };
        println!("tenth_value: {}", tenth_value);

        for i in &mut v {
            *i += 100
        }

        for i in &v {
            println!("{}", i)
        }
    }
}
