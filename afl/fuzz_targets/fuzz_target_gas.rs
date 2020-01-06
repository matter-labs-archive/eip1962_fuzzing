#[macro_use]
extern crate afl;
extern crate eth_pairings;

fn main() {
    fuzz!(|data: &[u8]| {
        let _ = eth_pairings::gas_meter::GasMeter::meter(&data);
    });
}