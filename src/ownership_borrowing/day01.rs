#[allow(dead_code)]
pub fn how_stack_heap_looks(){
    let s1 = String::from("hello");
    println!("address of s1 (stack struct)  : {:p}", &s1);
    println!("ptr inside s1 (heap address)  : {:p}", s1.as_ptr());
}


#[allow(dead_code)]
pub fn compare_stack_heap_clone_version() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("=== CLONE VERSION (s2 = s1.clone()) ===");
    println!("STACK                              HEAP");
    println!("┌──────────────────────┐");
    println!("│ s1 @ {:p}  │", &s1 as *const String);
    println!("│  ptr  ──────────────┼──────► ┌──────────────────┐");
    println!("│  len  = {:<13} │        │ {:p}   │", s1.len(), s1.as_ptr());
    println!("│  cap  = {:<13} │        │ h e l l o        │", s1.capacity());
    println!("└──────────────────────┘        └──────────────────┘");

    println!("┌──────────────────────┐");
    println!("│ s2 @ {:p}  │", &s2 as *const String);
    println!("│  ptr  ──────────────┼──────► ┌──────────────────┐  (different heap block)");
    println!("│  len  = {:<13} │        │ {:p}   │", s2.len(), s2.as_ptr());
    println!("│  cap  = {:<13} │        │ h e l l o        │", s2.capacity());
    println!("└──────────────────────┘        └──────────────────┘");
}


#[allow(dead_code)]
pub fn compare_stack_heap_move_version() {
    let s1 = String::from("hello");
   

    let s1_stack_addr = &s1 as *const String;
    let s1_heap_addr  = s1.as_ptr();
    let s1_len = s1.len();
    let s1_cap = s1.capacity();

    let s2 = s1; // s1 moved, can't use s1 after this

    println!("=== MOVE VERSION (s2 = s1) ===");
    println!("STACK                              HEAP");
    println!("┌──────────────────────┐");
    println!("│ s1 @ {:p}  │  (s1 moved — no longer valid)", s1_stack_addr);
    println!("│  ptr  ──────────────┼──────► ┌──────────────────┐");
    println!("│  len  = {:<13} │        │ {:p}   │", s1_len, s1_heap_addr);
    println!("│  cap  = {:<13} │        │ h e l l o        │", s1_cap);
    println!("└──────────────────────┘        └──────────────────┘");

    println!("┌──────────────────────┐");
    println!("│ s2 @ {:p}  │", &s2 as *const String);
    println!("│  ptr  ──────────────┼──────► ┌──────────────────┐  (SAME heap block as s1)");
    println!("│  len  = {:<13} │        │ {:p}   │", s2.len(), s2.as_ptr());
    println!("│  cap  = {:<13} │        │ h e l l o        │", s2.capacity());
    println!("└──────────────────────┘        └──────────────────┘");
}


#[allow(dead_code)]
pub fn compare_stack_heap(){
    compare_stack_heap_clone_version();
    println!("------------------------------------------");
    compare_stack_heap_move_version();
}


#[allow(dead_code)]
pub fn ownership_with_3_rules() {

    // ---- Rule 1: Each value has one owner ----
    let s1 = String::from("hello");
    println!("Rule 1: s1 is the owner of \"hello\" -> {}", s1);

    // ---- Rule 2: Only one owner at a time ----
    let s2 = s1; // ownership moves from s1 to s2
    // println!("{}", s1); // ❌ ERROR: s1 was moved, no longer valid
    println!("Rule 2: ownership moved -> s2 = {}", s2);

    // ---- Rule 3: When owner goes out of scope, value is dropped ----
    {
        let s3 = String::from("world");
        println!("Rule 3: s3 is alive inside this scope -> {}", s3);
    } // s3 goes out of scope here -> heap memory dropped/freed
    // println!("{}", s3); // ❌ ERROR: s3 does not exist outside this scope

    println!("Rule 3: s3 has been dropped, cannot use it here");
}


#[allow(dead_code)]
pub fn copy_vs_move_demo() {

    // ---- COPY TYPES (i32, bool, f64, char etc.) ----
    let x = 5;
    let y = x; // COPY, not move — i32 is a Copy type

    println!("addres of both: {:p}\n{:p}", &x, &y);

    println!("Copy type: x = {}, y = {}", x, y); // ✅ both valid

    // ---- MOVE TYPES (String, Vec, etc.) ----
    let s1 = String::from("hello");
    let s2 = s1; // MOVE — s1 is no longer valid

    // println!("{}", s1); // ❌ ERROR: value borrowed here after move
    println!("Move type: s2 = {}", s2);
}

// now time for borrowing and references

