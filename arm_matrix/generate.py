import os, shutil, sys
root="arm_matrix"; tst="arm_matrix_test"
# Only the generated trees go. `arm_matrix_test/tests/` is hand-written and must
# survive: an earlier version of this script removed it, the compile-fail suite
# vanished with it, and the workspace stayed green while nothing pinned the
# refusals any more.
shutil.rmtree(root, ignore_errors=True)
shutil.rmtree(f"{tst}/src", ignore_errors=True)
os.makedirs(f"{root}/src/impls", exist_ok=True); os.makedirs(f"{tst}/src", exist_ok=True)

def impl_fn(name, marker, arity):
    args = "_i: proc_macro::TokenStream" if arity==1 else "_a: proc_macro::TokenStream, _i: proc_macro::TokenStream"
    return (f"pub fn {name}({args}) -> proc_macro::TokenStream {{\n"
            f'    "pub fn {marker}_mark() -> &\'static str {{ \\"{marker}\\" }}"\n'
            f"        .parse()\n"
            f'        .expect("the emitted item is a fixed, valid function")\n'
            f"}}\n")

# form -> (spec template over {c} = cell name and {f} = implementation fn, placement)
FORMS = [
    ("bare",  "{f}",                    "scope"),
    ("use",   "use {f}",                "scope"),
    ("mod2",  "{c}::{f}",               "file"),
    ("emod2", "mod {c}::{f}",           "file"),
    ("umod2", "use {c}::{f}",           "file+decl"),
    ("nest",  "{c}::b::c::{f}",         "nested"),
    ("unest", "use {c}::b::c::{f}",     "nested+decl"),
    ("lit",   '"impls/{c}.rs"::{f}',    "impls"),
    ("crel",  '@"src/impls/{c}.rs"::{f}', "impls"),
    ("croot", "crate::{c}::{f}",          "file+decl"),
    ("sroot", "self::{c}::{f}",           "file+decl"),
]

def dname(cell): return "D" + "".join(w.capitalize() for w in cell.split("_"))
GROUPS = [
    ("f",   1, "function",  lambda c,s: f"include_proc_macro::proc_macro!({c} -> {s});"),
    ("a",   2, "attribute", lambda c,s: f"include_proc_macro::attr_macro!({c} -> {s});"),
    ("d",   1, "derive",    lambda c,s: f"include_proc_macro::derive_macro!(({dname(c)}) -> {s});"),
    ("da",  1, "derive",    lambda c,s: f"include_proc_macro::derive_macro!(({dname(c)}, attributes(alpha, beta)) -> {s});"),
    ("lf",  1, "function",  lambda c,s: f"include_proc_macro::macros!(function({c}) -> {s});"),
    ("la",  2, "attribute", lambda c,s: f"include_proc_macro::macros!(attribute({c}) -> {s});"),
    ("ld",  1, "derive",    lambda c,s: f"include_proc_macro::macros!(derive({dname(c)}) -> {s});"),
    ("lda", 1, "derive",    lambda c,s: f"include_proc_macro::macros!(derive({dname(c)}, attributes(alpha, beta)) -> {s});"),
    ("if",  1, "function",  lambda c,s: f"include_proc_macro::macros!(function -> {s});"),
    ("ia",  2, "attribute", lambda c,s: f"include_proc_macro::macros!(attribute -> {s});"),
]

scope, decls, invs, cells = [], [], [], []
for pfx, arity, shape, inv in GROUPS:
    for form, spec_t, place in FORMS:
        cell = f"{pfx}_{form}"
        infer = pfx in ("if", "ia")
        # A name-inferring entry over a bare in-scope function is refused by design:
        # the generated item would shadow the function it means to call. Those two
        # cells are pinned as compile-fail cases in tests/ui instead.
        if infer and form in ("bare", "use"):
            continue
        # A name-inferring entry takes the macro name from the path's last segment,
        # so the implementation function is what names the macro.
        implfn = cell if infer else ("imp" if not place.startswith("scope") else f"{cell}_fn")
        macro_name = implfn if infer else (dname(cell) if shape == "derive" else cell)
        marker = implfn if infer else cell
        spec = spec_t.format(c=cell, f=implfn)
        body = impl_fn(implfn, marker, arity)
        if place == "scope":
            scope.append(body)
        elif place.startswith("file"):
            open(f"{root}/src/{cell}.rs", "w").write(body)
            if place.endswith("decl"): decls.append(f"mod {cell};")
        elif place.startswith("nested"):
            inner = "pub mod b {\n    pub mod c {\n" + "\n".join(("        "+l if l else "") for l in body.splitlines()) + "\n    }\n}\n"
            open(f"{root}/src/{cell}.rs", "w").write(inner)
            if place.endswith("decl"): decls.append(f"mod {cell};")
        elif place == "impls":
            open(f"{root}/src/impls/{cell}.rs", "w").write(body)
        invs.append(inv(cell, spec))
        cells.append((marker, shape, macro_name))


# One `macros!` invocation with a hundred declarations in it. The assertion is that
# it compiles: the list form recurses once per declaration, and a splitter that
# munched tokens instead reached the default recursion limit of 128 at about twenty.
# The README states this number, so something has to hold it.
BULK = 100
open(f"{root}/src/bulk.rs","w").write(
    "//! A hundred implementations for the bulk declaration test.\n\n" +
    "".join(
        f"pub fn i{n}(_x: proc_macro::TokenStream) -> proc_macro::TokenStream "
        f"{{ proc_macro::TokenStream::new() }}\n"
        for n in range(BULK)))
bulk_decl = ("mod bulk;\n\n"
             "// Asserted by compiling: see bulk.rs.\n"
             "include_proc_macro::macros!(\n"
             + "".join(f"    function(bulk{n}) -> use bulk::i{n},\n" for n in range(BULK))
             + ");\n")

open(f"{root}/src/scope.rs","w").write("".join(scope))
open(f"{root}/src/lib.rs","w").write("\n".join(
  ["//! Every cell of the implementation-path grammar, against every way of declaring a macro.",
   "//!",
   "//! Nine path forms times ten declaration shapes. The set is generated rather than",
   "//! chosen, so a form that works for one kind and not another cannot slip through by",
   "//! being left out of the sample: it is a failing build here.",
   "//!",
   "//! Generated by `generate.py` in this directory. Regenerate rather than hand-editing.",
   "",
   "mod scope;",
   "#[allow(unused_imports)]",
   "use scope::*;",
   ""] + sorted(set(decls)) + [""] + invs + ["", bulk_decl]) + "\n")
open(f"{root}/Cargo.toml","w").write("""[package]
name = "arm_matrix"
publish = false
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
include_proc_macro = { path = "./.." }
""")

calls, asserts, seen = [], [], set()
for i,(marker, shape, mname) in enumerate(cells):
    if mname in seen: sys.exit(f"duplicate macro name {mname}")
    seen.add(mname)
    if shape == "function":   calls.append(f"arm_matrix::{mname}!();")
    elif shape == "attribute": calls.append(f"#[arm_matrix::{mname}]\nstruct P{i};")
    else:                      calls.append(f"#[derive(arm_matrix::{mname})]\nstruct S{i};")
    asserts.append(f'    assert_eq!({marker}_mark(), "{marker}");')

open(f"{tst}/src/lib.rs","w").write("\n".join(
  ["//! Every declaration in the matrix is invoked here and asserted to have produced",
   "//! its own marker, rather than merely to have compiled.",
   "",
   "#![allow(dead_code)]",
   ""] + calls + ["",
   "#[cfg(test)]",
   "mod tests {",
   "    use super::*;",
   "",
   "    #[test]",
   "    fn every_arm_of_the_grammar_produces_its_own_marker() {"] + asserts + ["    }","}"]) + "\n")
open(f"{tst}/Cargo.toml","w").write("""[package]
name = "arm_matrix_test"
publish = false
version = "0.1.0"
edition = "2021"

[dependencies]
arm_matrix = { path = "../arm_matrix" }
include_proc_macro = { path = "../" }

[dev-dependencies]
# Dev-only, so the published crate keeps its empty dependency list.
trybuild = "1.0"
""")
print(f"{len(cells)} cells, {len(asserts)} assertions")
