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

#[allow(dead_code)]
pub fn step1_what_is_reference() {
    let s1 = String::from("hello");
    let r = &s1; // r is a REFERENCE to s1

    println!("=== WHAT IS A REFERENCE ===");
    println!("STACK                              HEAP");
    println!("┌──────────────────────┐");
    println!("│ s1 @ {:p}  │", &s1 as *const String);
    println!("│  ptr  ──────────────┼──────► ┌──────────────────┐");
    println!("│  len  = {:<13} │        │ {:p}   │", s1.len(), s1.as_ptr());
    println!("│  cap  = {:<13} │        │ h e l l o        │", s1.capacity());
    println!("└──────────────────────┘        └──────────────────┘");

    println!("┌──────────────────────┐");
    println!("│ r  @ {:p}  │  (r itself lives on stack)", &r as *const &String);
    println!("│  value = {:p} ───────┼──────► points to s1 (above)", r);
    println!("└──────────────────────┘");

    println!();
    println!("s1 still usable   : {}", s1); // ✅ s1 valid hai
    println!("r usable too      : {}", r);  // ✅ r se bhi access ho raha
}


#[allow(dead_code)]
fn print_len(s: &String) {
    println!("--- inside print_len() ---");
    println!("address of s (param) : {:p}", &s as *const &String);
    println!("s points to          : {:p}", s);          // same as &s1 in main
    println!("s.as_ptr() (heap)    : {:p}", s.as_ptr());  // same heap as s1
    println!("len via reference    : {}", s.len());
}

#[allow(dead_code)]
pub fn step2_borrow_in_function() {
    let s1 = String::from("hello");

    println!("=== BEFORE FUNCTION CALL ===");
    println!("s1 @ {:p}", &s1 as *const String);
    println!("s1.as_ptr() = {:p}", s1.as_ptr());

    print_len(&s1); // borrow — ownership NOT moved

    println!("=== AFTER FUNCTION CALL ===");
    println!("s1 still usable: {}", s1); // ✅ s1 valid hai
}

#[allow(dead_code)]
fn push_world(s: &mut String) {
    println!("--- inside push_world() (BEFORE push) ---");
    println!("address of s (param) : {:p}", &s as *const &mut String);
    println!("s points to          : {:p}", s);
    println!("s.as_ptr() (heap)    : {:p}", s.as_ptr());
    println!("value via reference  : {}", s);

    s.push_str(", world");

    println!("--- inside push_world() (AFTER push) ---");
    println!("s.as_ptr() (heap)    : {:p}", s.as_ptr()); // same ya different?
    println!("value via reference  : {}", s);
}

#[allow(dead_code)]
pub fn step3_mutable_borrow() {
    let mut s1 = String::from("hello");

    println!("=== BEFORE FUNCTION CALL ===");
    println!("s1 @ {:p}", &s1 as *const String);
    println!("s1.as_ptr() = {:p}", s1.as_ptr());
    println!("s1.capacity() = {}", s1.capacity());
    println!("value = {}", s1);

    push_world(&mut s1); // mutable borrow

    println!("=== AFTER FUNCTION CALL ===");
    println!("s1.as_ptr() = {:p}", s1.as_ptr());
    println!("s1.capacity() = {}", s1.capacity());
    println!("value = {}", s1); // ✅ s1 modified
}


#[allow(dead_code)]
pub fn step4_borrow_rule_violation() {
    let mut s1 = String::from("hello");

    let r1 = &s1;       // immutable borrow #1
    let r2 = &s1;       // immutable borrow #2 — ✅ OK, multiple & allowed
  //  let r3 = &mut s1;   // mutable borrow — ❌ ERROR expected
   
    println!("r1: {}, r2: {}", r1, r2);

    let r3 = &mut s1;   // mutable borrow — will pass by NLL (Non-Lexical Lifetimes)

    r3.push_str(", world");
    println!("r3: {}", r3);
    println!("s1: {}", s1);


    // let r3 = &mut s1; // ❌ ERROR expected
    // r3.push_str(", world");
    // println!("r1: {}, r3: {}", r1, r3);

    println!("s1: {}", s1);
}



#[allow(dead_code)]
pub fn borrowing_reference_with_2_rules() {

    // ---- Rule 1: Many &T (immutable) OR one &mut T (mutable) at a time ----
    let mut s1 = String::from("hello");

    let r1 = &s1; // immutable borrow #1
    let r2 = &s1; // immutable borrow #2 -> OK, multiple immutable refs allowed
    println!("Rule 1: multiple immutable refs -> r1 = {}, r2 = {}", r1, r2);
    // r1, r2 are no longer used after this point (NLL marks them as dead)

    let r3 = &mut s1; // mutable borrow -> OK now, since r1, r2 are dead

   
     
    r3.push_str(", world");
    println!("Rule 1: one mutable ref -> r3 = {}", r3);

   

    println!("Rule 1: final value -> s1 = {}", s1);

    // ---- Rule 2: References must always be valid (no dangling references) ----
    let valid_ref;
    {
        let s4 = String::from("inner");
        valid_ref = &s4;
        println!("Rule 2: reference is valid inside this scope -> {}", valid_ref);
    } // s4 goes out of scope here -> heap memory dropped/freed
    // println!("{}", valid_ref); // ❌ ERROR: s4 does not live long enough, valid_ref would dangle

    println!("Rule 2: valid_ref cannot be used here, would be a dangling reference");
}

