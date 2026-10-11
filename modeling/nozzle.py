# Drives the hf-nozzle model (crates/hf-nozzle): build it with
#   CARGO_TARGET_DIR=~/tmp/target/nozzle cargo build --release -p hf-nozzle
# then evaluate specs here. A spec is a nested dict; missing fields take the
# crate's defaults (see crates/hf-nozzle/src/design.rs).

import json
import os
import subprocess
import sys

BIN = os.path.join(os.environ.get("CARGO_TARGET_DIR", os.path.expanduser("~/tmp/target/nozzle")),
                   "release", "hf-nozzle")


def evaluate(spec):
    out = subprocess.run([BIN, "-"], input=json.dumps(spec), capture_output=True, text=True)
    if out.returncode:
        raise RuntimeError(out.stderr)
    return json.loads(out.stdout)


def line(name, r):
    c = r["coil_ppm"]
    return (f"{name:34} v_e {r['ve_eff']:.3f}c {r['n_per_gw']:.2f} N/GW "
            f"side ({r['side'][0]*100:+5.2f},{r['side'][1]*100:+5.2f})% "
            f"plate {r['plate_ppm']:7.1f} side {r['side_ppm']:7.1f} ({r['ship_hits']}) "
            f"coil ring/sw/spoke {c['ring']:.0f}/{c['sweeper']:.0f}/{c['spoke']:.0f} ppm "
            f"fwd {r['forward']*100:4.2f}% magnet {r['magnet_t']:.0f} t "
            f"flux {r['peak_flux_mw_m2']:.0f} [{r['secs']:.0f}s]")


def show(name, spec):
    print(line(name, evaluate(spec)), flush=True)


if __name__ == "__main__":
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 20000
    show("design", {"run": {"n_ann": n}})
    show("design, forward cone", {"run": {"n_ann": n, "cone": 0.6, "seed": 2}})
