use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
	let physx_sys_path = Path::new("../../../neophysx-rs/physx-sys/");
	
	csbindgen::Builder::default()
		.input_bindgen_file(physx_sys_path.join("src/lib.rs"))
		.input_bindgen_file(physx_sys_path.join("src/physx_generated.rs"))
		.input_bindgen_file(physx_sys_path.join("src/generated/unix/structgen.rs"))
		.rust_file_header("use physx_sys::*;")
		.rust_method_prefix("neomagicphysx_")
		.csharp_entry_point_prefix("neomagicphysx_")
		.csharp_namespace("NeoMagicPhysX")
		.csharp_class_name("NativeMethods")
		.csharp_dll_name("libneomagicphysx")
		.csharp_class_accessibility("public")
		
		.method_filter(|x| !(x == "create_contact_callback" || x == "destroy_contact_callback"))
		
		.generate_to_file(
			"./src/gen/physx_ffi.rs",
			"../NeoMagicPhysX/NativeMethods.g.cs"
		)?;
	
//	// Link native lib
//	println!("cargo:rustc-link-search=native=natives");
//	println!("cargo:rustc-link-lib=static=physx_sys");
	
	Ok(())
}

/*
fn main() -> Result<(), Box<dyn Error>> {
    let mut b = csbindgen::Builder::new()
        .input_bindgen_file("src/physx/lib.rs")
        .input_bindgen_file("src/physx/physx_generated.rs");

    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    match (os.as_str(), arch.as_str()) {
        ("windows", "x86_64") => {
            b = b.input_bindgen_file("src/physx/x86_64-pc-windows-msvc/structgen.rs")
        }
        ("linux", "x86_64") => {
            b = b.input_bindgen_file("src/physx/x86_64-pc-windows-msvc/structgen.rs")
        }
        _ => panic!("unsupported (or not yet implemented) platform for physx"),
    }

    b.method_filter(|x| !(x == "create_contact_callback" || x == "destroy_contact_callback"))
        .rust_file_header("use super::physx_sys::*;")
        .rust_method_prefix("magicphysx_")
        .csharp_entry_point_prefix("magicphysx_")
        .csharp_namespace("MagicPhysX")
        .csharp_class_name("NativeMethods")
        .csharp_dll_name("libmagicphysx")
        .csharp_class_accessibility("public")
        .generate_to_file(
            "./src/physx_ffi.rs",
            "../MagicPhysX/NativeMethods.g.cs",
        )?;

    Ok(())
}
*/
