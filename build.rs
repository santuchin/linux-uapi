// build.rs
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {

	let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "x86_64".to_string());
	if target_arch != "x86_64" {
		panic!();
	}

	let kernel_root = Path::new("src/linux");
	if !kernel_root.exists() {
		panic!();
	}

	let tbl_path = kernel_root.join("arch/x86/entry/syscalls/syscall_64.tbl");
	if !tbl_path.exists() {
		panic!();
	}

	let out_dir = PathBuf::from("src");

	let tbl = fs::read_to_string(&tbl_path).unwrap();
	let syscalls = parse_syscall_tbl(&tbl);
	fs::write(&out_dir.join("sys.csv"), gen_sys_rs(&syscalls)).unwrap();


	generate_bindings();
}

fn generate_bindings() {

    let header = "src/linux/include/linux/syscalls.h";

	let outdir = env::var("OUT_DIR").unwrap();
	let outdir = "src/";

    let out_path = PathBuf::from(outdir);

    let bindings = bindgen::Builder::default()
        .header(header)
		.clang_arg("-Isrc/linux/include")
		.allowlist_function("sys_write")
        .generate()
		.unwrap();

    bindings
        .write_to_file(out_path.join("bindings.rs"))
		.unwrap()
}


fn parse_syscall_tbl(
	input: &str,
) -> Vec<
	(
		usize,
		&str,
		&str,
		Option<String>,
		Option<String>,
		bool,
	)
> {
	
	let mut result = Vec::new();

	for line in input.lines() {

		let line = line.trim();
		
		if line.is_empty() || line.starts_with('#') {
			continue;
		}
		
		// https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/x86/entry/syscalls/syscall_64.tbl

		let parts: Vec<&str> = line.split_whitespace().collect();
		
		if let &[number, abi, name, ref rest @ ..] = parts.as_slice() {

			let mut entry_point = rest.get(1);

			if entry_point == Some(&"-") {
				entry_point = None;
			}

			result.push(
				(
					number.parse::<usize>().unwrap(),
					abi,
					name,
					rest.get(0).map(|value| value.to_string()),
					entry_point.map(|value| value.to_string()),
					rest.get(2) == Some(&"noreturn"),
				)
			);
		}
	}
	
	result
}

fn gen_sys_rs(syscalls: &Vec<(usize, &str, &str, Option<String>, Option<String>, bool,)>) -> String {

	let mut result = String::new();
	
	for (number, abi, name, entry_point, compat_entry_point, noreturn) in syscalls {

		result.push_str(
			&format!("{},{},{},{},{},{}\n",
				number,
				abi,
				name,
				if let Some(this) = entry_point { this } else { "" },
				if let Some(this) = compat_entry_point { this } else { "" },
				if *noreturn { "1" } else { "0" },
			)
		);
	}

	result
}
