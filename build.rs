use glob::{glob, GlobError, PatternError};
use shaderc::OptimizationLevel;
use shaderc::{CompileOptions, ResolvedInclude};
use std::env;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: shaderc::ShaderKind,
}

impl ShaderData {
    pub fn load(src_path: PathBuf) -> BuildScriptResult<Self> {
        let extension = src_path
            .extension()
            .expect("File has no extension")
            .to_str()
            .expect("Extension cannot be converted to &str");
        let kind = match extension {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => panic!("Unknown shader type: {}", src_path.display()),
        };

        let src = read_to_string(src_path.clone())?;
        let spv_path = src_path.with_extension(format!("{}.spv", extension));

        Ok(Self {
            src,
            src_path,
            spv_path,
            kind,
        })
    }
}

fn main() -> BuildScriptResult<()> {
    let mut shader_paths = [
        glob("./src/shaders/*.vert")?,
        glob("./src/shaders/*.frag")?,
        glob("./src/shaders/*.comp")?,
    ];

    let shaders = shader_paths
        .iter_mut()
        .flatten()
        .map(|glob_result| ShaderData::load(glob_result?))
        .collect::<Vec<BuildScriptResult<_>>>()
        .into_iter()
        .collect::<BuildScriptResult<Vec<_>>>()?;
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut compile_options = CompileOptions::new().unwrap();
    compile_options.set_include_callback(
        |requested_source, _include_type, _requesting_source, include_depth| {
            if include_depth > 5 {
                panic!("Recursive include");
            }
            let mut dir = env::current_dir().unwrap();
            dir.push("src/shaders");
            dir.push(requested_source);
            Ok(ResolvedInclude {
                content: read_to_string(&dir).unwrap(),
                resolved_name: dir.to_str().unwrap().to_string(),
            })
        },
    );
    let target = std::env::var("TARGET").unwrap();
    let profile = std::env::var("PROFILE").unwrap();

    if profile.contains("debug") {
        compile_options.set_optimization_level(OptimizationLevel::Zero);
        compile_options.set_generate_debug_info();
    }

    if target.contains("wasm") {
        compile_options.add_macro_definition("IS_WEB", None);
    }

    for shader in shaders {
        // println!(
        //     "cargo:rerun-if-changed={}",
        //     shader.src_path.as_os_str().to_str().unwrap()
        // );
        println!("cargo:warning=Compiling {:?}", shader.src_path.to_str());
        let compiled = compiler.compile_into_spirv(
            &shader.src,
            shader.kind,
            shader.src_path.to_str().unwrap(),
            "main",
            Some(&compile_options),
        )?;
        println!("{}", compiled.get_warning_messages());
        write(shader.spv_path, compiled.as_binary_u8())?;
    }

    Ok(())
}

type BuildScriptResult<T> = Result<T, BuildScriptError>;

#[derive(Debug)]
enum BuildScriptError {
    ShaderInput(std::io::Error),
    GlobPattern(PatternError),
    Glob(GlobError),
    Shaderc(shaderc::Error),
}

impl From<std::io::Error> for BuildScriptError {
    fn from(error: std::io::Error) -> Self {
        BuildScriptError::ShaderInput(error)
    }
}

impl From<PatternError> for BuildScriptError {
    fn from(error: PatternError) -> Self {
        BuildScriptError::GlobPattern(error)
    }
}

impl From<GlobError> for BuildScriptError {
    fn from(error: GlobError) -> Self {
        BuildScriptError::Glob(error)
    }
}

impl From<shaderc::Error> for BuildScriptError {
    fn from(error: shaderc::Error) -> Self {
        BuildScriptError::Shaderc(error)
    }
}
