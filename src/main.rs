mod const_data;
use std::{fs, io::Write};
use git2::Repository;

use crate::const_data::*;


struct ProjectConfig {
    template_name: String,
    project_name: Option<String>,
    std: String,
    cmake_min_version: String,
    build_dir: String
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            template_name: String::from("default"),
            project_name: Some(String::from("MyProject")),
            std: String::from("17"),
            cmake_min_version: String::from("3.22"),
            build_dir: String::from("./bin")
        }
    }
}

fn main() {
    let all_args: Vec<String> = std::env::args().collect();
    let args = &all_args[1..];

    parse_commands(args);
}

fn parse_commands(cmds: &[String]) {
    if cmds.len() == 0 {
        print_help();
        return;
    }

    let command: String = cmds[0].to_lowercase();

    match command.as_str() {
        "new" => {
            let default_config = ProjectConfig::default();
            let template_name: &str = if cmds.len() == 1 || cmds[1].starts_with('-') {&default_config.template_name} else {cmds[1].as_str()};
            let config: ProjectConfig = ProjectConfig { 
                template_name: String::from(template_name), 
                project_name: find_parameter(cmds, &["-n", "--name"]),
                std: find_parameter(cmds, &["-s", "--std"]).unwrap_or(default_config.std),
                cmake_min_version: find_parameter(cmds, &["-c", "--cmake-min"]).unwrap_or(default_config.cmake_min_version),
                build_dir: String::from("./bin")
            };
            create_new_project(config);
        }
        _ => println!("There is no comand with the name '{}'", command),
    }
}

fn create_new_project(mut config: ProjectConfig) {
    let mut project_dir_path: String = String::from("./");

    match &config.project_name {
        Some(name) => {
            fs::create_dir_all(&name).expect("Failed to create project directory.");
            project_dir_path.push_str(&name);
        }
        None => config.project_name = ProjectConfig::default().project_name
    }

    
    let project_name: String = config.project_name.unwrap();
    let build_sh_src: String;
    let cmake_src: String;
    let gitignore_src: String;
    match config.template_name.as_str() {
        "default" => {
            build_sh_src = DEFAULT_EXE_BUILD_SH
                .replace("{{project_name}}", &project_name)
                .replace("{{build_dir}}", &config.build_dir);
            cmake_src = DEFAULT_EXE_CMAKE_LISTS
                .replace("{{project_name}}", &project_name)
                .replace("{{cmake_min_version}}", &config.cmake_min_version)
                .replace("{{cpp_standard}}", &config.std);
            gitignore_src = GITIGNORE_SRC.replace("{{build_dir_name}}", &config.build_dir.replace("./", ""));

        }
        "lib" | "library" => {
            build_sh_src = DEFAULT_LIB_BUILD_SH
                .replace("{{project_name}}", &project_name);
            cmake_src = DEFAULT_LIB_CMAKE_LISTS
                .replace("{{project_name}}", &project_name)
                .replace("{{cmake_min_version}}", &config.cmake_min_version)
                .replace("{{cpp_standard}}", &config.std);
            gitignore_src = GITIGNORE_SRC.replace("{{build_dir_name}}", &config.build_dir.replace("./", ""));
        }
        _ => {
            println!("There is no template with the name '{}'", config.template_name);
            return;
        }
    }

    let mut build_file = fs::File::create(format!("{}/{}", &project_dir_path, "build.sh")).expect("Failed to create 'build.sh' file.");
    build_file.write_all(build_sh_src.as_bytes()).expect("Failed to write into 'build.sh' file.");


    let mut cmake_file = fs::File::create(format!("{}/{}", &project_dir_path, "CMakeLists.txt")).expect("Failed to create 'CMakeLists.txt' file.");
    cmake_file.write_all(cmake_src.as_bytes()).expect("Failed to write into 'CMakeLists' file.");

    let src_dir_path = format!("{}/{}", &project_dir_path, "src");
    fs::create_dir_all(&src_dir_path).expect("Failed to create 'src' directory.");
    let mut main_file = fs::File::create(format!("{}/{}", src_dir_path, "main.cpp")).expect("Failed to create 'main.cpp' file.");
    main_file.write_all(DEFAULT_MAIN_SRC.as_bytes()).expect("Failed to write into 'main.cpp' file.");

    fs::create_dir_all(format!("{}/{}", &project_dir_path, "external/lib/win")).expect("Failed to create win lib directory.");
    fs::create_dir_all(format!("{}/{}", &project_dir_path, "external/lib/linux")).expect("Failed to create linux lib directory.");

    match Repository::init(&project_dir_path) {
        Ok(repository) => repository,
        Err(e) => panic!("Failed to create git repository: {}", e),
    };

    let mut gitignore_file = fs::File::create(format!("{}/{}", project_dir_path, ".gitignore")).expect("Failed to create gitignore file.");
    gitignore_file.write_all(gitignore_src.as_bytes()).expect("Failed to write into gitingore file.");

    println!("Project '{}' was successfully created! (template: '{}')", &project_name, &config.template_name);
}

fn print_help() {
    println!("Commands:");
    println!("    new");
    println!("    help");

    println!("Flags:");
    println!("    '-n', '--name' : Name of the project,");
    println!("    '-s', '--std' : Required version of C++ standard,");
    println!("    '-c', '--cmake-min' : Minimal required version of CMake,");
}

fn find_parameter<'a>(cmds: &'a[String], flags: &[&str]) -> Option<String> {
    let mut parameter: Option<String> = Option::None;

    for f in flags {
        for (i, c) in cmds.iter().enumerate() {
            if c == f && i < cmds.len() - 1 {
                parameter = Some(cmds[i + 1].to_string());
            }
        }
    }

    return parameter;
}