fn collatz(szam: u64, lepes: &mut u64, max: &mut u64) {
    if *max < szam {
        *max = szam;
    }
    *lepes +=1; // lepes.checked_add(1).expect("Túlcsordult a számláló!");
    if szam != 1 {
        if szam % 2 == 0 {
            collatz(szam / 2, lepes, max);
        } else {
            collatz(3 * szam + 1, lepes, max); // checked_mul, checked_add
        }
    }
}

fn main() {
    let maxcount = std::env::args()
        .nth(1)
        .expect("Argumentum 1 szám")
        .parse()
        .expect("Az argumentum szám kell legyen!");
    let mut maxnum = 0;
    for szam in 1..maxcount {
        let mut lepes = 0;
        let mut tmpmax = 0;
        collatz(szam, &mut lepes, &mut tmpmax);
        if maxnum < tmpmax {
            maxnum = tmpmax;
            println!("Lépés: {} Szám: {}, max: {}", lepes, szam, maxnum);
        }
    }
}
