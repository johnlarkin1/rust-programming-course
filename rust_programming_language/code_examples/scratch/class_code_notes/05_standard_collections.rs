

/*
Collections                 | What is it?                   | C++           | C#    | Java      | Python            
============================================================================================================
Vec<T>                      | Dynamic growable array        | vector        | List  | ArrayList | list             |
VecDeque<T>                 | Double-ended queue            | deque         | -     | ArrayDeque| collections.deque
LinkedList<T>               | Doubly linked list            | list          | LinkedList | LinkedList | -
BinaryHeap<T> where T : Ord | Max heap                      | priority_queue| -     | PriorityQueue | heapq
HashMap<K,V> where          | Dictionary (key-value table)  | unordered_map | Dictionary      |               | dict
    K : Eq-Hash             | 
BTreeMap<K,V> where         | Sorted dictionary             | map           | SortedDictionary      |               | -
    K : Ord                 |
HashSet<T> where            | Hash table                    | unordered_set | HashSet       |               | set
    T: Eq + Hash            |        
BTreeSet<T> where           | Sorted set                    | set           | SortedSet      |               |
    T : Order               |    


* Most important one is probably the Vec<T>
* Vec is going to be key
* Doubly linked list 
    - Removal from collection 
    - Not contiguous in memory
* Binary heap / max heap
    - Priority queue
* Map duh
* BTreeMap
    - Sorted version
    - if you have a requirement that the keys are sorted etc
* HashTable 
    - Unique elements 

*/


// Vec(tor)
fn vectors() {
    // a needs to be mutable
    let mut a = Vec::new(); 
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);

    // But what's the type of the index into the array?
    let idx:i32 = 0;
    // Can't use a signed index into an array
    // Always going to be usize
    let good_idx:usize = 0;
    a[good_idx] = 4;

    for x in &a {
        println!("{}", x);
    }

    // push and pop
    let last_elem = a.pop(); // Option returned

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

// HashMap

use std::collections::HashMap;
fn hash_maps() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("A square has {} sides", shapes["square".into()]);

    // Entry
    // If it's there, then update 
    // If it's not, then insert it in there
    shapes.entry("circle".into()).or_insert(1);

    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

// HashSet
use std::thread;
use std::time;
use std::collections::HashSet;

fn hash_sets() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("greeks = {:?}", greeks);

    // Nothing happens on dup insert
    greeks.insert("delta");
    println!("greeks = {:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("Nice, added vega");
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("Nice, removed delta");
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!( 
        "is {:?} a subset of {:?}? {}",
        _2_8, _1_10, _2_8.is_subset(&_1_10)
    );

    // disjoint
    println!( 
        "is {:?} disjoint of {:?}? {}",
        _1_5, _6_10, _1_5.is_disjoint(&_6_10)
    );

    // union and intersection as well
    // difference = items in the first set, but not in the second set
    // symantic_diff = union minus intersection
}

// Iterators 
fn iterators() {
    // Bit more flexibility 
    let mut vec = vec![3,2,1];
    
    // for x in vec {
    //     println!("{}", x);
    // }
    // can't use vec again because you've basically done a move
    // You've moved the value 

    for x in &vec {
        // Now you're taking a borrow 
        println!("{}", *x);
    }

    // Iter will give you immutable references 

    for x in vec.iter() {
        println!("we got {}", x);
    }

    for x in vec.iter_mut() {
        *x +=2 ;
        // Changes the vec
    }
    println!("vec = {:?}", vec);

    // Go backwards? Sure 
    for x in vec.iter().rev() {
        println!("in reverse: {}", x);
    }

    let mut vec2 = vec![1,2,3];
    // let it = vec.into_iter();
    vec2.extend(vec);
    println!("{:?}", vec2);
    // println!("{:?}", vec); // can't do this because we've borrowed vec
}



fn main() {
    vectors();
    hash_maps();
    hash_sets();
    iterators();
}
