use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::combine::combine;
use puzzle_check::common::dataclass::{Attribute, Composition, Coordinate, Element, Structure};
use puzzle_check::common::function::{power_set, progress_size};
use puzzle_check::common::initialize::initialize;
use puzzle_check::common::relationship::{relationship, Relationship, D, H, M, V};
use std::collections::HashSet;

fn main() {
    let n = 2;
    let m = 2;
    let NaN = 10000;
    let (P, C, Ep, Ec) = initialize(n, m);
    // ----------------------------------------------------------------------
    let R: HashSet<Relationship> = vec![H, D, V].into_iter().collect();
    let G = combine(R, &Ep);
    println!("1. -----------------------------",);
    println!("{:?}", 2usize.pow(423));
    let power_G = power_set(&G);
    println!("2. -----------------------------",);

    let P_domain = [NaN];
    let C_domain = [0, 1, 2, 3, 4];
    let Ep_domain = [0, 1];
    let Ec_domain = [NaN];
    let G_domain = [NaN];

    let P_domain_size = P_domain.len();
    let C_domain_size = C_domain.len();
    let Ep_domain_size = Ep_domain.len();
    let Ec_domain_size = Ec_domain.len();
    let G_domain_size = G_domain.len();

    let P_size = P.len();
    let C_size = C.len();
    let Ep_size = Ep.len();
    let Ec_size = Ec.len();
    let G_size = (G_domain_size + 1).pow(G.len() as u32) as u64;

    let pb = ProgressBar::new(
        progress_size(P_domain_size, P_size)
            * progress_size(C_domain_size, C_size)
            * progress_size(Ep_domain_size, Ep_size)
            * progress_size(Ec_domain_size, Ec_size)
            * G_size,
    );
    print!(
        "{:?}",
        progress_size(P_domain_size, P_size)
            * progress_size(C_domain_size, C_size)
            * progress_size(Ep_domain_size, Ep_size)
            * progress_size(Ec_domain_size, Ec_size)
            * G_size
    )
}
