fn main() {
    int_sort();
    float_sort();
    struct_sort();
}

fn int_sort() {
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort(); // 昇順のソートをしてくれる。

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    vec.sort_unstable(); // sortより高速だが、等しい要素の順序を保持しない

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

fn float_sort() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap()); // sort_byとpartial_cmpを使うとfloatのソートができる

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

// Eq, Ord, PartialEq, PartialOrdを設定することで構造体でソートできるようになる
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn struct_sort() {
    let mut people_list = vec![
        Person::new("jun".to_string(), 31),
        Person::new("rei".to_string(), 26),
        Person::new("takuma".to_string(), 29),
    ];

    people_list.sort();

    assert_eq!(
        people_list,
        vec![
            Person::new("jun".to_string(), 31),
            Person::new("rei".to_string(), 26),
            Person::new("takuma".to_string(), 29),
        ]
    );

    people_list.sort_by(|a, b| b.age.cmp(&a.age)); // b.cmp(&a)は昇順、a.cmp(&b)は降順

    assert_eq!(
        people_list,
        vec![
            Person::new("jun".to_string(), 31),
            Person::new("takuma".to_string(), 29),
            Person::new("rei".to_string(), 26),
        ]
    );
}
