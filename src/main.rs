use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "init" {
        println!("Usage: {} init <project_name>", args[0]);
        return;
    }

    let project_name = &args[2];
    let project_dir = Path::new(project_name);

    if project_dir.exists() {
        println!("Error: Project directory '{}' already exists", project_name);
        return;
    }

    // Create project directory
    fs::create_dir(project_dir).expect("Failed to create project directory");

    // Create src directory
    let src_dir = project_dir.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src directory");

    // Read and process Cargo.toml template
    let cargo_toml_template = fs::read_to_string("src/templates/rust-service/Cargo.toml")
        .expect("Failed to read Cargo.toml template");
    let cargo_toml = cargo_toml_template
        .replace("{{project-name}}", project_name)
        .replace("{{project-name}}_lambda", &format!("{}_lambda", project_name));
    fs::write(project_dir.join("Cargo.toml"), cargo_toml).expect("Failed to create Cargo.toml");

    // Read and process main.rs template
    let main_rs_template = fs::read_to_string("src/templates/rust-service/src/main.rs")
        .expect("Failed to read main.rs template");
    let main_rs = main_rs_template.replace("{{project-name}}", project_name);
    fs::write(src_dir.join("main.rs"), main_rs).expect("Failed to create main.rs");

    // Read and process lib.rs template
    let lib_rs = fs::read_to_string("src/templates/rust-service/src/lib.rs")
        .expect("Failed to read lib.rs template");
    fs::write(src_dir.join("lib.rs"), lib_rs).expect("Failed to create lib.rs");

    // Create common module
    let common_dir = src_dir.join("common");
    fs::create_dir(&common_dir).expect("Failed to create common directory");

    // Read and process common/mod.rs template
    let common_mod_rs = fs::read_to_string("src/templates/rust-service/src/common/mod.rs")
        .expect("Failed to read common/mod.rs template");
    fs::write(common_dir.join("mod.rs"), common_mod_rs).expect("Failed to create common/mod.rs");

    // Read and process common/types.rs template
    let types_rs = fs::read_to_string("src/templates/rust-service/src/common/types.rs")
        .expect("Failed to read common/types.rs template");
    fs::write(common_dir.join("types.rs"), types_rs).expect("Failed to create common/types.rs");

    // Read and process common/validation.rs template
    let validation_rs = fs::read_to_string("src/templates/rust-service/src/common/validation.rs")
        .expect("Failed to read common/validation.rs template");
    fs::write(common_dir.join("validation.rs"), validation_rs).expect("Failed to create common/validation.rs");

    // Create routes module
    let routes_dir = src_dir.join("routes");
    fs::create_dir(&routes_dir).expect("Failed to create routes directory");

    // Read and process routes/mod.rs template
    let routes_mod_rs = fs::read_to_string("src/templates/rust-service/src/routes/mod.rs")
        .expect("Failed to read routes/mod.rs template");
    fs::write(routes_dir.join("mod.rs"), routes_mod_rs).expect("Failed to create routes/mod.rs");

    // Create process module
    let process_dir = src_dir.join("process");
    fs::create_dir(&process_dir).expect("Failed to create process directory");

    // Read and process process/mod.rs template
    let process_mod_rs = fs::read_to_string("src/templates/rust-service/src/process/mod.rs")
        .expect("Failed to read process/mod.rs template");
    fs::write(process_dir.join("mod.rs"), process_mod_rs).expect("Failed to create process/mod.rs");

    // Read and process process/processing.rs template
    let processing_rs = fs::read_to_string("src/templates/rust-service/src/process/processing.rs")
        .expect("Failed to read process/processing.rs template");
    fs::write(process_dir.join("processing.rs"), processing_rs).expect("Failed to create process/processing.rs");

    // Create services module
    let services_dir = src_dir.join("services");
    fs::create_dir(&services_dir).expect("Failed to create services directory");

    // Read and process services/lambda.rs template
    let lambda_rs_template = fs::read_to_string("src/templates/rust-service/src/services/lambda.rs")
        .expect("Failed to read services/lambda.rs template");
    let lambda_rs = lambda_rs_template.replace("{{project-name}}", project_name);
    fs::write(services_dir.join("lambda.rs"), lambda_rs).expect("Failed to create services/lambda.rs");

    // Read and process Makefile template
    let makefile_template = fs::read_to_string("src/templates/rust-service/Makefile")
        .expect("Failed to read Makefile template");
    let makefile = makefile_template.replace("{{project-name}}", project_name);
    fs::write(project_dir.join("Makefile"), makefile).expect("Failed to create Makefile");

    println!("Project '{}' created successfully!", project_name);
    println!("To get started:");
    println!("  cd {}", project_name);
    println!("  cargo build");
    println!("  cargo run");
}
