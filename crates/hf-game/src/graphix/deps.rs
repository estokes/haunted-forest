use anyhow::Result;
use arcstr::ArcStr;
use fxhash::FxHashMap;
use graphix_compiler::ExecCtx;
use graphix_package::{IndexSet, Package};
use graphix_rt::{GXExt, GXRt};
use netidx_core::path::Path;

pub(crate) struct RegisterResult {
    pub root: ArcStr,
}

pub(crate) fn register<X: GXExt>(
    ctx: &mut ExecCtx<GXRt<X>, X::UserEvent>,
    modules: &mut FxHashMap<Path, ArcStr>,
) -> Result<RegisterResult> {
    let mut root_mods = IndexSet::new();
    graphix_package_core::P::register(ctx, modules, &mut root_mods)?;
    graphix_package_array::P::register(ctx, modules, &mut root_mods)?;
    graphix_package_str::P::register(ctx, modules, &mut root_mods)?;
    graphix_package_map::P::register(ctx, modules, &mut root_mods)?;
    graphix_package_sys::P::register(ctx, modules, &mut root_mods)?;
    graphix_package_gui::P::register(ctx, modules, &mut root_mods)?;
    let mut parts = Vec::new();
    for name in &root_mods {
        if name == "core" {
            parts.push(format!("mod core;\nuse core"));
        } else {
            parts.push(format!("mod {name}"));
        }
    }
    Ok(RegisterResult {
        root: ArcStr::from(parts.join(";\n")),
    })
}
