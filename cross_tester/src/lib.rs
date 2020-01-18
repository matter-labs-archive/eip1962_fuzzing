#![allow(dead_code)]

#![cfg_attr(feature = "benchmarks", feature(test))]

extern crate eth_pairings;
extern crate eth_pairings_cpp;
extern crate hex;

#[cfg(all(feature = "benchmarks", test))]
mod bench;

mod run_on_csv;
mod run_on_fuzzer_inputs;

fn run(data: &[u8]) {
    if data.len() < 1 {
        return;
    }

    // println!("Input = {}", hex::encode(&data));
    let native = eth_pairings::public_interface::API::run(&data);
    let cpp = eth_pairings_cpp::run(&data);
    match (native, cpp) {
        (Ok(n), Ok(c)) => {
            if n != c {
                println!("Input = {}", hex::encode(&data));
                println!("Native result = {}, C++ result = {}", hex::encode(&n), hex::encode(&c));
                panic!("Native result = {}, C++ result = {}", hex::encode(&n), hex::encode(&c));
            } else {
                println!("Native and C++ results coincide");
                // println!("Native and C++ results coincide on {}", hex::encode(&n));
            }
        },
        (Err(n), Err(c)) => {
            println!("Native and C++ results coincide on error: {:?}, {:?}", n, c);
        },
        (Ok(n), Err(c)) => {
            println!("Input = {}", hex::encode(&data));
            println!("Native result = {}, while C++ returned error {:?}", hex::encode(&n), c);
            panic!("Native result = {}, while C++ returned error {:?}", hex::encode(&n), c);
        },
        (Err(n), Ok(c)) => {
            println!("Input = {}", hex::encode(&data));
            println!("Native result returned error {:?}, while C++ returned {}", n, hex::encode(&c));
            panic!("Native result returned error {:?}, while C++ returned {}", n, hex::encode(&c));
        }
    }
}

fn run_with_op(data: &[u8]) {
    if data.len() < 1 {
        return;
    }
    let native_op = eth_pairings::public_interface::OperationType::from_u8(data[0]);
    let cpp_op = eth_pairings_cpp::OperationType::from_u8(data[0]);

    match (native_op, cpp_op) {
        (Some(n), Some(c)) => {
            assert!(n.as_u8() == c.as_u8());
        },
        (None, None) => {
            return
        },
        _ => {
            panic!("Difference in parsing of operation types");
        }
    }

    let native_op = native_op.unwrap();
    let cpp_op = cpp_op.unwrap();
    
    let native = eth_pairings::public_interface::perform_operation(native_op, &data[0..]);
    let cpp = eth_pairings_cpp::perform_operation(cpp_op, &data[0..]);
    match (native, cpp) {
        (Ok(n), Ok(c)) => {
            if n != c {
                println!("Input = {}", hex::encode(&data));
                println!("Native result = {}, C++ result = {}", hex::encode(&n), hex::encode(&c));
                panic!("For op type {:?}: Native result = {}, C++ result = {}", native_op, hex::encode(&n), hex::encode(&c));
            } else {
                println!("Native and C++ results coincide");
                // println!("Native and C++ results coincide on {}", hex::encode(&n));
            }
        },
        (Err(n), Err(c)) => {
            println!("Native and C++ results coincide on error: {:?}, {:?}", n, c);
        },
        (Ok(n), Err(c)) => {
            println!("Input = {}", hex::encode(&data));
            println!("Native result = {}, while C++ returned error {:?}", hex::encode(&n), c);
            panic!("For op type {:?}: Native result = {}, while C++ returned error {:?}", native_op, hex::encode(&n), c);
        },
        (Err(n), Ok(c)) => {
            println!("Input = {}", hex::encode(&data));
            println!("Native result returned error {:?}, while C++ returned {}", n, hex::encode(&c));
            panic!("For op type {:?}: Native result returned error {:?}, while C++ returned {}", native_op, n, hex::encode(&c));
        }
    }
}

fn run_gas_with_op(data: &[u8]) {
    if data.len() < 1 {
        return;
    }
    let native_op = eth_pairings::public_interface::OperationType::from_u8(data[0]);
    let cpp_op = eth_pairings_cpp::OperationType::from_u8(data[0]);

    match (native_op, cpp_op) {
        (Some(n), Some(c)) => {
            assert!(n.as_u8() == c.as_u8());
        },
        (None, None) => {
            return
        },
        _ => {
            panic!("Difference in parsing of operation types");
        }
    }

    let native_op = native_op.unwrap();
    let cpp_op = cpp_op.unwrap();

    println!("Op = {:?}", native_op);
    
    let native = eth_pairings::gas_meter::meter_operation(native_op, &data[0..]);
    let cpp = eth_pairings_cpp::meter_operation(cpp_op, &data[0..]);
    match (native, cpp) {
        (Ok(n), Ok(c)) => {
            if n != c {
                println!("Input = {}", hex::encode(&data));
                println!("Native result = {}, C++ result = {}", n, c);
                panic!("For op type {:?}: Native result = {}, C++ result = {}", native_op, n, c);
            } else {
                println!("Native and C++ results coincide");
                // println!("Native and C++ results coincide on {}", hex::encode(&n));
            }
        },
        (Err(n), Err(c)) => {
            println!("Native and C++ results coincide on error: {:?}, {:?}", n, c);
        },
        (Ok(n), Err(c)) => {
            println!("Input = {}", hex::encode(&data));
            println!("Native result = {}, while C++ returned error {:?}", n, c);
            panic!("For op type {:?}: Native result = {}, while C++ returned error {:?}", native_op, n, c);
        },
        (Err(n), Ok(c)) => {
            println!("Input = {}", hex::encode(&data));
            println!("Native result returned error {:?}, while C++ returned {}", n, c);
            panic!("For op type {:?}: Native result returned error {:?}, while C++ returned {}", native_op, n, c);
        }
    }
}


#[test]
fn cross_check_on_input() {
    let filename = "slow-unit-f3a20e0db984a4a6759a80b01af14b7b6ae30367";
    use std::time::Instant;
    use std::io::Read;
    use std::fs::File;
    let mut file = File::open(&format!("../artifacts/fuzz_target_api/{}", filename)).expect("must open");
    let mut input_data = vec![];
    file.read_to_end(&mut input_data).expect("must read");
    assert!(input_data.len() != 0);
    let now = Instant::now();
    self::run(&input_data[..]);
    let elapsed = now.elapsed().as_micros();
    println!("Api call taken in {} micros", elapsed);
}

#[test]
fn cross_check_on_hongg_input() {
    let filename = "SIGSEGV.EXC_BAD_ACCESS.PC.00007ffeefb3fa78.STACK.00000015553112ca.ADDR.00007ffeefb3fa78.fuzz";
    use std::time::Instant;
    use std::io::Read;
    use std::fs::File;
    let mut file = File::open(&format!("../honggfuzz/hfuzz_workspace/fuzz_target_compare/{}", filename)).expect("must open");
    let mut input_data = vec![];
    file.read_to_end(&mut input_data).expect("must read");
    assert!(input_data.len() != 0);
    let now = Instant::now();
    self::run(&input_data[..]);
    let elapsed = now.elapsed().as_micros();
    println!("Api call taken in {} micros", elapsed);
}