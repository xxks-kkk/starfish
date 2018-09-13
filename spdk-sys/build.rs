extern crate bindgen;
extern crate make_cmd;
extern crate toml;

//use make_cmd::gnu_make;
use std::env;
//use std::fmt::Write;
//use std::fs::File;
//use std::io::Read;
use std::path::PathBuf;
//use toml::Value;

fn main_run() {
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("spdk_config.properties");

    // let make_output = gnu_make()
    //     .arg(format!("ENV_PATH={:?}", out_path))
    //     .output()
    //     .expect("make failed");

    // let mut f = File::open(out_path).expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");

    // println!("cargo:warn={}", contents);

    // let value = contents.parse::<Value>().unwrap();
    // let libs = value["LIBS"].as_str().unwrap();

    // let mut output = String::new();
    // for s in libs.split(" ") {
    //     write!(&mut output, "\"-C\", \"link-arg={}\",\n", s).unwrap();
    // }

    // println!("cargo:warn={}", output);
    // println!("cargo:rerun-if-changed=./build.rs");

    let spdk_path = env::var("SPDK_DIR").unwrap_or("/tmp/spdk/include".to_string());
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let generator = Generator { spdk_path, output_path };

    generator.generate("nvme");
    generator.generate("event");
    generator.generate("bdev");
    generator.generate("env");
    generator.generate("blob_bdev");
    generator.generate("blob");
    generator.generate("log");
    generator.generate("io_channel")  
}

struct Generator {
    spdk_path: String,
    output_path: PathBuf
}

impl Generator {
    fn generate(&self, name: &str) {
        let mut codegen_config = bindgen::CodegenConfig::nothing();
        codegen_config.functions = true;
        codegen_config.types = true;

        let bindings = bindgen::Builder::default()
            .header(format!("{}/spdk/{}.h", self.spdk_path, name))
            .derive_default(true)
            .with_codegen_config(codegen_config)
            // Figure out how to make sure the includes are working ok
            .clang_arg(format!("-I{}", self.spdk_path))
            // If there are linking errors and the generated bindings have weird looking
            // #link_names (that start with \u{1}), the make sure to flip that to false.
            .trust_clang_mangling(false)
            .rustfmt_bindings(true)
            .generate()
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        bindings
            .write_to_file(self.output_path.join(format!("spdk_{}_bindings.rs", name)))
            .expect("Couldn't write bindings!");
    }
}

fn main() {
    // Uncomment to regenerate bindings
    main_run();
    println!("cargo:rerun-if-changed=./build.rs");
}
