use heck::ToUpperCamelCase;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    env,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
struct RawTld {
    #[serde(rename = "Domain")]
    domain: String,
    #[serde(rename = "Type")]
    tld_type: String,
    #[serde(rename = "Manager")]
    manager: String,
    #[serde(rename = "Country")]
    country: String,
}

fn find_data_file(start: &Path) -> PathBuf {
    let mut dir = start.to_path_buf();
    loop {
        let candidate = dir.join("data/tlds.json");
        if candidate.exists() {
            return candidate;
        }
        if !dir.pop() {
            panic!("Could not find data/tlds.json above {}", start.display());
        }
    }
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let json_path = find_data_file(Path::new(&manifest_dir));
    // https://users.rust-lang.org/t/51852/2
    println!("cargo:rerun-if-changed={}", json_path.display());

    // Read TLD data file
    let raw = fs::read_to_string(&json_path)
        .unwrap_or_else(|e| panic!("Could not read {}: {}", json_path.display(), e));
    let entries: BTreeMap<String, RawTld> =
        serde_json::from_str(&raw).expect("Could not parse data/tlds.json");

    // Discover type variants from data
    let mut variant_names: BTreeMap<String, String> = BTreeMap::new();
    for entry in entries.values() {
        variant_names
            .entry(entry.tld_type.clone())
            .or_insert_with(|| entry.tld_type.to_upper_camel_case());
    }

    // Write enum for types of TLD
    let mut enum_src = String::new();
    enum_src += "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n";
    enum_src += "pub enum Type {\n";
    for name in variant_names.values() {
        enum_src += &format!("    {},\n", name);
    }
    enum_src += "}\n\n";

    // Build the TLD map
    let mut builder = phf_codegen::Map::new();
    let mut struct_literals: BTreeMap<String, String> = BTreeMap::new();

    for (key, entry) in &entries {
        let variant = &variant_names[&entry.tld_type];
        let mut lit = String::new();
        write!(
            lit,
            "Tld {{ domain: {domain:?}, r#type: Type::{typ}, manager: {manager:?}, country: {country:?} }}",
            domain = entry.domain,
            typ = variant,
            manager = entry.manager,
            country = entry.country,
        )
        .unwrap();
        struct_literals.insert(key.clone(), lit);
    }

    for (key, lit) in &struct_literals {
        builder.entry(key.as_str(), lit);
    }

    let map_src = format!(
        "pub static TLD_MAP: phf::Map<&'static str, Tld> = {};\n",
        builder.build()
    );

    // Write output
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("map.rs");
    fs::write(&dest, enum_src + &map_src).unwrap();
}
