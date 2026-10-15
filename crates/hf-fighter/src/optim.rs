//! Nelder-Mead minimisation in two dimensions.

pub fn nelder_mead(
    f: impl Fn([f64; 2]) -> f64,
    start: [f64; 2],
    step: [f64; 2],
    iters: usize,
) -> ([f64; 2], f64) {
    let mut s = [start, [start[0] + step[0], start[1]], [start[0], start[1] + step[1]]];
    let mut v = [f(s[0]), f(s[1]), f(s[2])];
    for _ in 0..iters {
        let mut idx = [0, 1, 2];
        idx.sort_by(|a, b| v[*a].total_cmp(&v[*b]));
        s = [s[idx[0]], s[idx[1]], s[idx[2]]];
        v = [v[idx[0]], v[idx[1]], v[idx[2]]];
        let c = [(s[0][0] + s[1][0]) / 2., (s[0][1] + s[1][1]) / 2.];
        let along = |t: f64| [c[0] + t * (s[2][0] - c[0]), c[1] + t * (s[2][1] - c[1])];
        let r = along(-1.);
        let fr = f(r);
        if fr < v[0] {
            let e = along(-2.);
            let fe = f(e);
            (s[2], v[2]) = if fe < fr { (e, fe) } else { (r, fr) };
        } else if fr < v[1] {
            (s[2], v[2]) = (r, fr);
        } else {
            let k = along(0.5);
            let fk = f(k);
            if fk < v[2] {
                (s[2], v[2]) = (k, fk);
            } else {
                for i in 1..3 {
                    s[i] = [(s[i][0] + s[0][0]) / 2., (s[i][1] + s[0][1]) / 2.];
                    v[i] = f(s[i]);
                }
            }
        }
    }
    let best = (0..3).min_by(|a, b| v[*a].total_cmp(&v[*b])).unwrap();
    (s[best], v[best])
}
