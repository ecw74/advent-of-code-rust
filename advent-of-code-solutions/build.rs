use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if cfg!(feature = "codegen") {
        // Change the year and
        // run 'cargo build --features codegen'
        let year = "0";

        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let day_template_path = Path::new(&manifest_dir).join("template/day.rs.tpl");
        let day_template_content =
            fs::read_to_string(&day_template_path).expect("Failed to read template/day.rs.tpl");

        let day_impl_template_path = Path::new(&manifest_dir).join("template/day_impl.rs.tpl");
        let day_impl_template_content = fs::read_to_string(&day_impl_template_path)
            .expect("Failed to read template/day_impl.rs.tpl");

        let mod_rs_path = Path::new(&manifest_dir)
            .join("src")
            .join("solutions")
            .join(format!("aoc{}", year))
            .join("mod.rs");

        let mut mod_rs_content = String::new();

        for day in 1..=26 {
            let day_str = format!("{:02}", day);

            // Zielpfad für dayXX.rs
            let day_output_path = Path::new(&manifest_dir)
                .join("src")
                .join("solutions")
                .join(format!("aoc{}", year))
                .join(format!("day{}.rs", day_str));

            let day_generated_content = day_template_content
                .replace("{day}", &day_str)
                .replace("{year}", year);

            fs::write(&day_output_path, day_generated_content)
                .expect(&format!("Failed to write day{}.rs", day_str));

            let day_impl_output_path = Path::new(&manifest_dir)
                .join("src")
                .join("solutions")
                .join(format!("aoc{}", year))
                .join(format!("day{}_impl.rs", day_str));

            let day_impl_generated_content = day_impl_template_content
                .replace("{day}", &day_str)
                .replace("{year}", year);

            fs::write(&day_impl_output_path, day_impl_generated_content)
                .expect(&format!("Failed to write day{}_impl.rs", day_str));

            mod_rs_content.push_str(&format!("pub use day{}::Day{};\n", day_str, day_str));
            mod_rs_content.push_str(&format!("mod day{};\n", day_str));
            mod_rs_content.push_str(&format!("mod day{}_impl;\n", day_str));
        }

        fs::write(&mod_rs_path, mod_rs_content).expect("Failed to write mod.rs");

        println!("cargo:rerun-if-changed=template/day.rs.tpl");
        println!("cargo:rerun-if-changed=template/day_impl.rs.tpl");
    }
}
