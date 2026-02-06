#![allow(dead_code)]

use std::i32;

fn print_binary(n: i32, desc: &str) {
    print!("0x{:08X} = {:11} = ", n as u32, n);
    for i in (0..=31).rev() {
        print!("{}", (n >> i) & 1);
        if i % 8 == 0 && i != 0 {
            print!(" ");
        }
    }
    println!("  {}", desc);
}

fn demonstrate_negation_edge_cases() {
    println!("\n=== NEGATION EDGE CASES ===");

    let negated: i32 = i32::MIN;
    print_binary(negated, "INT_MIN");
    print_binary(negated.wrapping_neg(), "INT_MIN wrapping_neg(), because negation panics with overflow");

    let val1 = negated + 1;
    print_binary(val1, "INT_MIN+1");
    print_binary(-val1, "INT_MIN+1 negation");

    let val2: i32 = i32::MAX;
    print_binary(val2, "INT_MAX");
    print_binary(-val2, "INT_MAX negation");

    let val3: i32 = i32::MAX - 1;
    print_binary(val3, "INT_MAX-1");
    print_binary(-val3, "INT_MAX-1 negation");
}

fn demonstrate_left_shift_overflow() {
    println!("\n=== LEFT SHIFT OVERFLOW (UNDEFINED BEHAVIOR) ===");

    let val1: i32 = i32::MAX / 2;
    print_binary(val1, " - starting value");

    let result1: i32 = val1 << 1; // This is still defined
    print_binary(result1, "<< 1");

    let result2: i32 = val1 << 2; // overflow, use .wrapping_shl(2);
    print_binary(result2, "<< 2");

    let result3: i32 = val1 << 3; // overflow, use .wrapping_shl(3);
    print_binary(result3, "<< 3");

    println!();
    // Example: Negating a left-shifted value near edge
    let val: i32 = 0x40000000; // 2^30
    print_binary(val, " - starting value 2^30");

    let shifted: i32 = val << 1; //.wrapping_shl(1);
    print_binary(shifted, "<< 1 (creates INT_MIN)");

    let u_val: u32 = 0x40000000;
    let u_shifted: u32 = u_val << 1;
    print_binary(u_shifted as i32, "unsigned shift << 1");
}

fn demonstrate_right_shift_signed() {
    println!("\n=== RIGHT SHIFT OF NEGATIVE VALUES (IMPLEMENTATION-DEFINED) ===");

    let negative: i32 = -16;
    print_binary(negative, " - starting value");

    for i in 1..=6 {
        let shifted: i32 = negative >> i;
        let buf = format!(">> {}: ", i);
        print_binary(shifted, &buf);
    }
}

#[allow(arithmetic_overflow)]
fn demonstrate_shift_by_negative_or_large() {
    println!("\n=== SHIFT BY NEGATIVE OR >= 32 (UNDEFINED BEHAVIOR) ===");

    let val: i32 = 0x12345678;
    print_binary(val, " - starting value");

    let result: i32 = val.wrapping_shl(32);
    print_binary(result, "<< 32 (expected 0?)");

    let result: i32 = val.wrapping_shr(32);
    print_binary(result, "[ >> 32] replaced with");

    let result: i32 = val.wrapping_shl(33);
    print_binary(result, "<< 33");

    let result: i32 = val.wrapping_shr(33);
    print_binary(result, ">> 33");

    // In C: val << -2 is undefined behavior
    // In Rust: wrapping_shl with negative reinterprets as unsigned, so -2i32 as u32 = 4294967294
    // which mod 32 = 30
    let result: i32 = val.wrapping_shl((-2i32) as u32);
    print_binary(result, "<< -2");

    let result: i32 = val.wrapping_shr((-2i32) as u32);
    print_binary(result, ">> -2");

    let result: i32 = val.wrapping_shr(2);
    print_binary(result, ">> 2");

    let result: i32 = val.wrapping_shl(0);
    print_binary(result, "<< 0");

    let result: i32 = val.wrapping_shr(0);
    print_binary(result, ">> 0");
}

fn demonstrate_norm32() {
    println!("\n=== NORM32 FUNCTION ===");
    println!("norm32(x) = number of left shifts to get MSB into position 30\n");

    let test_values: [usize; 13] = [
        0,
        -1i32 as usize,
        1,
        -2i32 as usize,
        i32::MIN as usize,
        i32::MAX as usize,
        0x40000000,
        0x20000000,
        0x00000100,
        0x00010000,
        0x7FFFFFFF,
        0x00000001,
        0x80000000
    ];

    for val in test_values {
        let result = norm32(val as u32);
        print!("norm32() -> {:2}  for ", result);
        print_binary(val as i32, "");
    }
}

fn norm32(a: u32) -> i16 {
    (if (a & 0x80000000) != 0 { !a } else { a }).leading_zeros() as i16 - 1
}

fn main() {
    // println!("=================================================");
    // println!("CRITICAL BIT SHIFT NUANCES IN Rust (SIGNED 32-BIT)");
    // println!("=================================================");

    // demonstrate_negation_edge_cases();
    // demonstrate_left_shift_overflow();
    // demonstrate_right_shift_signed();
    // demonstrate_shift_by_negative_or_large();

    demonstrate_norm32();
}
