///https://www.programming-idioms.org/cheatsheet/Rust
///
///
/// training mf

///Print Hello World
fn one() {
    println!("Hello World");
}

///Print Hello 10 times
fn two() {
    for _ in 0..10 {
        println!("hello");
    }
}

///Create a procedure
fn three(name: &str) {
    dbg!(name);
}

///Create a function which returns the square of an integer
fn four(x: u32) -> u32 {
    x * x
}

///Create a 2D Point data structure
struct Point {
    x: f64,
    y: f64,
}

///Iterate over list values
fn six() {
    let arr: [i32; 5] = [0, 2, 3, 4, 5];
    for idx in arr.iter() {
        dbg!(idx);
    }
}

///Iterate over list indexes and values
fn seven() {
    let arr: [i32; 5] = [0, 2, 3, 4, 5];
    for (idx, value) in arr.iter().enumerate() {
        dbg!(idx, value);
    }
}

///Initialize a new map (associative array)
fn eight() {
    use std::collections::BTreeMap;
    let mut x = BTreeMap::new();
    x.insert("one", 1);
    x.insert("two", 2);

    use std::collections::HashMap;
    let map_hash: HashMap<&str, i32> = [("one", 1), ("two", 2)].iter().cloned().collect();

    dbg!(x);
    dbg!(map_hash);
}

///Create a Binary Tree data structure
#[derive(Debug)]
struct BinTree<T> {
    value: T,
    left: Option<Box<BinTree<T>>>,
    right: Option<Box<BinTree<T>>>,
}

///Shuffle a list
fn ten() {
    use rand::prelude::*;
    use rand::rngs::StdRng;
    let arr: [i32; 5] = [0, 2, 3, 4, 5];

    // let mut rng = StdRng::new().unwrap();
    // rng.shuffle(&mut arr);
    let mut rng = thread_rng();
    let idx: f64 = rng.gen();
    let mut y = arr;
    y.shuffle(&mut rng);

    dbg!(y);
}

///Pick a random element from a list
fn eleven() {
    use rand::prelude::*;
    let arr: [i32; 5] = [0, 2, 3, 4, 5];

    let choice = arr[rand::thread_rng().gen_range(0, arr.len())];
    //let choice = thread_rng().choose(&arr).unwrap();
    dbg!(choice);
}

///Check if list contains a value
fn twelve() {
    let arr: [i32; 5] = [0, 2, 3, 4, 5];

    let cnt = arr.contains(&4);

    let cnt_any = arr.iter().any(|item| item == &4);
    dbg!(cnt);
    dbg!(cnt_any);
}

///Iterate over map keys and values
fn thirteen() {
    use std::collections::HashMap;

    let mut mymap = HashMap::new();
    mymap.insert(1, "one");
    mymap.insert(2, "two");

    for (k, v) in mymap {
        dbg!(k, v);
    }
}

///Pick uniformly a random floating point number in [a..b)
fn fourteen() {
    use rand::prelude::*;

    let a = 10.5;
    let b = 20.5;

    let uniform_num = thread_rng().gen_range(a, b);
    dbg!(uniform_num);
}

///Pick uniformly a random integer in [a..b]
fn fifteen() {
    use std::ops::RangeInclusive;

    let a = 10;
    let b = 20;
    let between = RangeInclusive::new(a, b);
    //let mut rng = rand::thread_rng().gen_range(a,b);
    between.contains(&15);
}

///Create a Tree data structure
#[derive(Debug)]
struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
}

///Depth-first traversing of a tree
impl<T> Node<T> {
    pub fn dfs<F: Fn(&T)>(&self, f: F) {
        self.dfs_helper(&f);
    }
    fn dfs_helper<F: Fn(&T)>(&self, f: &F) {
        (f)(&self.value);
        for child in &self.children {
            child.dfs_helper(f)
        }
    }
}

///Reverse a list
fn nineteen() {
    let mut arr: [i32; 5] = [0, 2, 3, 4, 5];

    // let rv: Vec<&i32> = arr.iter().rev().collect();
    arr.reverse();
    dbg!(arr);
    // dbg!(rv);
}

///	Implement a function search which looks for item x in a 2D matrix m.
/// Return indices i, j of the matching cell.
///Think of the most idiomatic way in the language to return the two values at the same time.
fn twenty<T: Eq>(m: &Vec<Vec<T>>, x: &T) -> Option<(usize, usize)> {
    for (i, row) in m.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if *column == *x {
                return Some((i, j));
            }
        }
    }
    None
}

///Swap values
fn twentyone() {
    use std::mem::swap;

    let mut a = 10;
    let mut b = 20;

    swap(&mut a, &mut b);
    dbg!(a, b);
}

///Convert string to integer
fn twentytwo() {
    // let s = "one";
    let s = "1";

    let i = s.parse::<i32>().unwrap();
    let e: i32 = s.parse().unwrap_or(0);

    let f = match s.parse::<i32>() {
        Ok(i) => i,
        Err(e) => -1,
    };

    dbg!(i, e, f);
}

///Convert real number to string with 2 decimal places
fn twentythree() {
    let rnum = 1;

    let s = format!("{:.2}", rnum);
    dbg!(s);
}

///Assign to string the japanese word ネコ
fn twentyfive() {
    let s = "ネコ";
    dbg!(s);
}

///Send a value to another thread
fn twentyfive_one() {
    use std::sync::mpsc::channel;
    use std::thread;

    let (send, recv) = channel();
    thread::spawn(move || loop {
        let msg = recv.recv().unwrap();
        println!("Hello, {:?}", msg);
    });
    send.send("Nathi Nioce").unwrap();
    dbg!(&send);
    drop(send);
}

///Create a 2-dimensional array
fn twentysix() {
    let N = 5;
    let M = 10;
    let mut x = vec![vec![0.0f64; N]; M];
    // let mut y = [[0.0; N] ; M];
}

///Create a 3-dimensional array
fn twentyseven() {
    let N = 10;
    let P = 5;
    let M = 12;
    let x = vec![vec![vec![0.0f64; P]; N]; M];
}

///Sort by a property
fn twentyeight() {
    let items = [1, 2, 3, 4, 5];
    // items.sort_by(|a,b|a.p.cmp(&b.p));
    // items.sort_by_key(|ite| item.p);
}

///Remove item from list, by its index
fn twentynine() {
    let mut items: Vec<i32> = vec![1, 2, 3, 4, 5];

    items.remove(4);
    dbg!(items);
}
fn f(i: usize) {
    dbg!(i);
}
///Parallelize execution of 1000 independent tasks
fn thirty() {
    use std::thread;

    let threads: Vec<_> = (0..1000).map(|i| thread::spawn(move || f(i))).collect();

    // (0..10000).into_par_iter().for_each(f);?
}

///Recursive factorial (simple)
fn thirtyone() {
    f(2);
    fn f(n: u32) -> u32 {
        println!("{}", &n);
        if n < 2 {
            1
        } else {
            n * f(n - 1)
        }
    }
}

///Integer exponentiation by squaring
fn thirtytwo() {
    exp(2, 2);
    fn exp(x: u64, n: u64) -> u64 {
        match n {
            0 => 1,
            1 => x,
            i if i % 2 == 0 => exp(x * x, n / 2),
            _ => x * exp(x * x, (n - 1) / 2),
        }
    }
}

///Atomically read and update variable
fn thirtythree() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let mutex = Arc::new(Mutex::new(0));
    let c_mutex = mutex.clone();

    thread::spawn(move || {
        *c_mutex.lock().unwrap() = 10;
    })
    .join()
    .expect("thread::spawn failed");
    dbg!(&*mutex);
}

///Create a Set of objects
fn thirtyfour() {
    use std::collections::HashSet;

    let x: HashSet<i32> = HashSet::new();
}
///First-class function : compose
fn thirtyfive() {

    // fn compose<'a, A,B,C,G,F>(f: F, g: G) -> Box<Fn(A) -> C + 'a>
    //     where F: 'a + Fn(A) -> B, G: 'a + Fn(B) -> C {
    //         Box::new(move |x| g(f(x)))
    //     }
}
///Currying
fn thirtyseven() {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
    let add5 = move |x| add(5, x);
    dbg!(add5(2));
}
///Extract a substring
fn thirtyeight() {
    let s = "HelloBro";

    // let t = s.graphemes(true).skip(i).take(j - i).collect::<String>();
}

///Check if string contains a word
fn thirtynine() {
    let s = "HelloBro";

    let word = s.contains("Bro");
    dbg!(word);
}

fn forty() {}
///Reverse a string
fn fortyone() {
    let s = "HelloBro";

    let rev: Vec<_> = s.chars().rev().collect();
    dbg!(rev);
}

///Continue outer loop
fn fortytwo() {
    let a = [1, 2, 3, 4, 5];
    let b = [6, 7, 3, 8, 1];

    'outer: for va in &a {
        for vb in &b {
            if va == vb {
                continue 'outer;
            }
        }
        dbg!(va);
    }
}

///Break outer loop
fn fortythree() {
    let a = [1, 2, 3, 4, 5];
    let b = [6, 7, 3, -1, 8, 1];

    'outer: for v in &a {
        'inner: for i in &b {
            if i < &0 {
                dbg!(i);
                break 'outer;
            }
        }
    }
}

///Insert element in list
fn fortyfour() {
    let mut a = vec![1, 2, 3];

    a.insert(1, 10);
    dbg!(a);
}
///Pause execution for 5 seconds
fn fortyfive() {
    use std::thread;
    use std::time;

    println!("waiting for 5 secs");
    thread::sleep(time::Duration::from_secs(5));
}

//Extract beginning of string (prefix)
fn fortysix() {
    let s = "HelloBro";

    let t = s[..5].to_string();
    dbg!(t);
}
///Extract string suffix
fn fortyseven() {
    let s = "HelloBro";

    let t = s[s.len() - 5..].to_string();
    dbg!(t);
}

///Multi-line string literal
fn fortyeight() {
    let s = "line 1
    line 2
    line 3";

    let r = r#"huey
    Dewey
    Louie"#;

    dbg!(s);
    dbg!(r);
}

///Split a space-separated string
fn fortynine() {
    let s = "hello bro, wa mpona na?";

    let chunks: Vec<_> = s.split_whitespace().collect();
    dbg!(chunks);
}

///Make an infinite loop
fn fifty() {
    loop {
        println!("bro");
        break;
    }
}
///Check if map contains key
fn fiftyone() {
    use std::collections::HashMap;

    let mut m = HashMap::new();
    m.insert(1, "one");
    m.insert(2, "two");

    let check = m.contains_key(&4);
    dbg!(check);
}
///Check if map contains value
fn fiftytwo() {
    use std::collections::HashMap;

    let mut m = HashMap::new();
    m.insert(1, "one");
    m.insert(2, "two");

    let v = String::from("two");
    let checkv = m.values().any(|&x| *x == v);
    dbg!(checkv);
}

///oin a list of strings
fn fiftythree() {
    // let s1 = String::from("hello");
    // let s2 = String::from("world");
    let s1 = ["hello", "bro"];
    // let s2 = ["bro"];

    let sjoin = s1.join(",");
    dbg!(sjoin);
}
///Compute sum of integers
fn fiftyfour() {
    let a = [1, 2, 3];

    let sum: u32 = a.iter().sum();
    let sum_alt = a.iter().sum::<u32>();

    dbg!(sum);
    dbg!(sum_alt);
}
///Convert integer to string
fn fiftyfive() {
    let integ = 10;

    let conv = integ.to_string();
    let conv_format = format!("{}", integ);
    dbg!(conv);
    dbg!(conv_format);
}
///Launch 1000 parallel tasks and wait for completion
fn fiftysix() {
    use std::thread;
    fn f(i: usize) {
        dbg!(i);
    }
    let threads: Vec<_> = (0..1000).map(|i| thread::spawn(move || f(i))).collect();

    for t in threads {
        //dbg!(&t);
        t.join();
    }
}
///Filter list
fn fiftyseven() {
    let x = [1, 2, 3, 4];
    let p = 3;
    let y: Vec<i32> = x
        .iter()
        .filter(|item: &&i32| **item == p)
        .cloned()
        .collect();
    dbg!(y);
}

///Extract file content to a string
fn fiftyeight() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    // use std::path::PathBuf;

    // let mut path = PathBuf::new();
    // path.push("/tmp/");
    // path.push("tmp");
    // let mut file_create = File::create("foo.txt")?;
    let mut file = File::open("foo.txt")?;
    file.write_all(b"hello bro")?;
    let mut contents = String::new();
    let read = file.read_to_string(&mut contents)?;
    println!("{:?}", read);
    dbg!(read);
    Ok(())
}
///Write to standard error stream
fn fiftynine() {
    let x = -2;
    eprintln!("{} is negative", x);
}
///Read command line argument
fn sixty() {
    use std::env;

    let first_arg = env::args().skip(1).next();
    let fallback = "".to_owned();
    let x = first_arg.unwrap_or(fallback);
}
///Get current date
fn sixtyone() {
    use std::time;

    let d = time::SystemTime::now();
    dbg!(d);
}

//Find substring position
fn sixtytwo() {
    // let s = From::String("hello bro");
    let s = "hello bro";
    let y = "e";
    let subs = s.find(y).unwrap();
    dbg!(subs);
}

//Replace fragment of a string
fn sixtythree() {
    let s = "hello bro";
    let x2 = s.replace("hello", "Ola");
    dbg!(x2);
}

///Big integer : value 3 power 247
fn sixtyfour() {
    extern crate rand;
    extern crate num_bigint as bigint;
    // extern crate num_traits;
    extern crate num;

    use bigint::{RandBigInt, ToBigInt};
    use num::pow;

    // let mut rng = rand::thread_rng();
    // let low = -10000.to_bigint().unwrap();
    // let high = 10000.to_bigint().unwrap();
    // let b = rng.gen_bigint_range(&low, &high);

    // let mut rng = rand::thread_rng();
    let a = 3.to_bigint().unwrap();
    //let rand_big_num = rng.gen_bigint(3);
    // println!(
    //     "a is {} and rand is {} and product of them is {}",
    //     &a,
    //     &rand_big_num,
    //     &a * &rand_big_num
    // );
    let x = pow(a, 247);
    dbg!(x);
}

fn main() {
    //six();
    //seven()
    //eight();
    //ten();
    // eleven();
    // twelve();
    // thirteen();
    // fourteen();
    // nineteen();
    // twentyone();
    // twentytwo();
    // twentythree();
    // twentyfive_one();
    // twentynine();
    // thirtyone();
    // thirtythree();
    // thirtyseven();
    // thirtynine();
    // fortyone();
    // fortytwo();
    // fortythree();
    // fortyfour();
    // fortyfive();
    // fortysix();
    // fortyseven();
    // fortyeight();
    // fortynine();
    // fiftytwo();
    // fiftythree();
    // fiftyfour();
    // fiftyfive();
    // fiftysix();
    // fiftyseven();
    //Some(fiftyeight());
    // fiftynine();
    // sixtyone();
    // sixtytwo();
    sixtyfour();
}
