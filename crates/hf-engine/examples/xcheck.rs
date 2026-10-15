//! Points for the Cantera cross-check: HP equilibrium of dust feeds with
//! ammonia or methane, the soot it holds, and an equilibrium expansion.
use hf_engine::gas::{eq_hp, eq_tp, Diluent, Feed};
use hf_engine::thermo::{el, sp};
fn main() -> anyhow::Result<()> {
    for (d, x, p) in [
        (Diluent::Ammonia, 0., 10e6),
        (Diluent::Ammonia, 1., 10e6),
        (Diluent::Ammonia, 3.28, 50e6),
        (Diluent::Ammonia, 12., 50e6),
        (Diluent::Methane, 0.96, 10e6),
        (Diluent::Methane, 1., 10e6),
        (Diluent::Methane, 7.32, 10e6),
    ] {
        let f = Feed::blend(&[(1., Feed::dust()), (x, d.feed(0.))]);
        let g = eq_hp(&f, p, None)?;
        let e = g.eq_s(g.s(), 1.5e6)?;
        println!(
            "{} {x} {p} {} {} {} {} {} {} {} {} {}",
            d,
            f.b[el::H],
            f.b[el::N],
            f.b[el::C],
            f.h,
            g.t,
            g.x(sp::H),
            g.soot(),
            e.t,
            g.h() - e.h()
        );
    }
    // Soot at fixed temperature and pressure.
    let f = Diluent::Methane.feed(0.);
    for (t, p) in [(1500., 1e5), (2500., 1e6), (3500., 1e5)] {
        let g = eq_tp(&f.b, t, p, None)?;
        println!("tp {t} {p} {} {} {}", f.b[el::H], f.b[el::C], g.soot());
    }
    Ok(())
}
