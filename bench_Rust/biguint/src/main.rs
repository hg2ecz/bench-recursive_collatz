use num_bigint::{BigUint,ToBigUint};

fn collatz(szam: BigUint, lepes: &mut u128, max: &mut BigUint) {
    if *max < szam {
        *max = szam.clone();
    }
    *lepes = lepes.checked_add(1).expect("Túlcsordult a számláló!");
    if szam != 1u8.into() {
        if &szam & 1.to_biguint().unwrap() == 0u8.into() {
            collatz(szam / 2.to_biguint().unwrap(), lepes, max);
        } else {
            collatz(3.to_biguint().unwrap() * szam + 1.to_biguint().unwrap(), lepes, max); // checked_mul, checked_add
        }
    }
}

fn main() {
    let maxcount = std::env::args()
        .nth(1)
        .expect("Argumentum 1 szám")
        .parse::<BigUint>()
        .expect("Az argumentum szám kell legyen!");
    let mut maxnum = 0u8.into();
    //for szam in 1.into()..maxcount {
    let mut szam: BigUint = 1u8.into();
    while szam < maxcount {
        let mut lepes = 0;
        let mut tmpmax = 0u8.into();
        collatz(szam.clone(), &mut lepes, &mut tmpmax);
        if maxnum < tmpmax {
            maxnum = tmpmax;
            println!("Lépés: {} Szám: {}, max: {}", lepes, szam, maxnum);
        }
        szam += 1u8.to_biguint().unwrap();
    }
}
