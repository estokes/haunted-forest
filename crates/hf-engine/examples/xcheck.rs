//! Points for the Cantera cross-check: HP equilibrium of dust/ammonia feeds
//! and an equilibrium expansion.
use hf_engine::{
    gas::{eq_hp, Feed},
    thermo::{el, sp},
};
fn main() -> anyhow::Result<()> {
    for (x, p) in
        [(0., 10e6), (1., 10e6), (2., 10e6), (3.28, 50e6), (5.6, 50e6), (12., 50e6)]
    {
        let f = Feed::blend(&[(1., Feed::dust()), (x, Feed::ammonia(0.))]);
        let g = eq_hp(&f, p, None)?;
        let e = g.eq_s(g.s(), 1.5e6)?;
        println!(
            "{x} {p} {} {} {} {} {} {} {}",
            f.b[el::H],
            f.b[el::N],
            f.h,
            g.t,
            g.x(sp::H),
            e.t,
            g.h() - e.h()
        );
    }
    Ok(())
}
