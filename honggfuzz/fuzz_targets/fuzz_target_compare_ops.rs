#[macro_use] extern crate honggfuzz;

extern crate eth_pairings;
extern crate eth_pairings_cpp;
extern crate hex;

fn main() {
    // Here you can parse `std::env::args and 
    // setup / initialize your project

    // You have full control over the loop but
    // you're supposed to call `fuzz` ad vitam aeternam
    loop {
        // The fuzz macro gives an arbitrary object (see `arbitrary crate`)
        // to a closure-like block of code.
        // For performance reasons, it is recommended that you use the native type
        // `&[u8]` when possible.
        // Here, this slice will contain a "random" quantity of "random" data.
        fuzz!(|data: &[u8]| {
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
                    return;
                },
                _ => {
                    panic!("Difference in parsing of operation types");
                }
            }

            let native_op = native_op.unwrap();
            let cpp_op = cpp_op.unwrap();

            let native = eth_pairings::gas_meter::meter_operation(native_op, &data[1..]);
            let cpp = eth_pairings_cpp::meter_operation(cpp_op, &data[1..]);
            match (native, cpp) {
                (Ok(n), Ok(c)) => {
                    if n != c {
                        // println!("Input = {}", hex::encode(&data));
                        println!("Native result = {}, C++ result = {}", n, c);
                        panic!("Native result = {}, C++ result = {}", n, c);
                    } else {
                        if n == 0 {
                            panic!("Gas estimate is zero");
                        }
                        // println!("Native and C++ gas results coincide");
                    }
                },
                (Err(n), Err(c)) => {
                    if n == eth_pairings::public_interface::ApiError::Overflow {
                        panic!("Overflow when estimated gas");
                    }
                    // println!("Native and C++ results coincide on error: {:?}, {:?}", n, c);
                },
                (Ok(n), Err(c)) => {
                    // println!("Input = {}", hex::encode(&data));
                    println!("Native result = {}, while C++ returned error {:?}", n, c);
                    panic!("Native result = {}, while C++ returned error {:?}", n, c);
                },
                (Err(n), Ok(c)) => {
                    // println!("Input = {}", hex::encode(&data));
                    println!("Native result returned error {:?}, while C++ returned {}", n, c);
                    panic!("Native result returned error {:?}, while C++ returned {}", n, c);
                }
            }
            
            let native = eth_pairings::public_interface::perform_operation(native_op, &data[1..]);
            let cpp = eth_pairings_cpp::perform_operation(cpp_op, &data[1..]);
            match (native, cpp) {
                (Ok(n), Ok(c)) => {
                    if n != c {
                        println!("Native result = {}, while C++ result = {}", hex::encode(&n), hex::encode(&c));
                        panic!("Native result = {}, C++ result = {}", hex::encode(&n), hex::encode(&c));
                    } else {
                        // println!("Native and C++ results coincide");
                        // println!("Native and C++ results coincide on {}", hex::encode(&n));
                    }
                },
                (Err(_n), Err(_c)) => {
                    // println!("Native and C++ results coincide on error: {:?}, {:?}", n, c);
                },
                (Ok(n), Err(c)) => {
                    // println!("Input = {}", hex::encode(&data));
                    println!("Native result = {}, while C++ returned error {:?}", hex::encode(&n), c);
                    panic!("Native result = {}, while C++ returned error {:?}", hex::encode(&n), c);
                },
                (Err(n), Ok(c)) => {
                    // println!("Input = {}", hex::encode(&data));
                    println!("Native result returned error {:?}, while C++ returned {}", n, hex::encode(&c));
                    panic!("Native result returned error {:?}, while C++ returned {}", n, hex::encode(&c));
                }
            }
        });
    }
}