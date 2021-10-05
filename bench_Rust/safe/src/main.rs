fn collatz(szam: u64, lepes: &mut u64, max: &mut u64) {
    if *max < szam {
        *max = szam;
    }
    *lepes = lepes.checked_add(1).expect("Túlcsordult a számláló!");
    if szam != 1 {
        if szam % 2 == 0 {
            collatz(szam / 2, lepes, max);
        } else {
            collatz(
                szam.checked_mul(3)
                    .expect("Szorzás túlcsordulás")
                    .checked_add(1)
                    .expect("Összeadás túlcsordulás"),
                lepes,
                max,
            );
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
