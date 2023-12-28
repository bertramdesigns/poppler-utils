use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

// &str, String, and Path are excepted, all returning a Path
pub enum PopplerFile {
    Path(PopplerFilePath),
    Buffer(PopplerFileBuffer),
}
pub struct PopplerFilePath {
    path: PathBuf,
}

pub struct PopplerFileBuffer {
    buffer: Vec<u8>,
}
pub trait AsPopplerPath {
    fn as_poppler_path(self) -> PopplerFile;
}

impl AsPopplerPath for &str {
    fn as_poppler_path(self) -> PopplerFile {
        PopplerFile::Path(PopplerFilePath {
            path: PathBuf::from(self),
        })
    }
}

impl AsPopplerPath for String {
    fn as_poppler_path(self) -> PopplerFile {
        PopplerFile::Path(PopplerFilePath {
            path: PathBuf::from(self),
        })
    }
}

impl AsPopplerPath for PathBuf {
    fn as_poppler_path(self) -> PopplerFile {
        PopplerFile::Path(PopplerFilePath { path: self })
    }
}

impl AsPopplerPath for &Path {
    fn as_poppler_path(self) -> PopplerFile {
        PopplerFile::Path(PopplerFilePath {
            path: PathBuf::from(self),
        })
    }
}

pub trait AsPopplerBuffer {
    fn as_poppler_buffer(self) -> PopplerFile;
}

impl AsPopplerBuffer for Vec<u8> {
    fn as_poppler_buffer(self) -> PopplerFile {
        PopplerFile::Buffer(PopplerFileBuffer { buffer: self })
    }
}

pub(crate) async fn run_program(
    file: PopplerFile,
    prog_name: &str,
    parsed_options: Vec<String>,
) -> Result<String, std::io::Error> {
    let exe_path = get_path_to_executable(prog_name);

    // error if there is not a valid path to the executable.
    // Check the hard coded folder structures.
    if let Err(_) = exe_path {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get_path_to_executable",
        ));
    }

    let mut handle = Command::new(exe_path.unwrap());

    // determine if the file is a path or a fileBuffer and configure accordingly
    match file {
        PopplerFile::Buffer(_) => {
            handle.arg("-");
            handle.args(parsed_options);
            handle.stdin(Stdio::piped());
            handle.stdout(Stdio::piped());
            handle.stderr(Stdio::piped());
        }
        PopplerFile::Path(ref file) => {
            handle.arg(file.path.clone());
            handle.args(parsed_options);
            handle.stdout(Stdio::piped());
            handle.stderr(Stdio::piped());
        }
    }

    let child = handle.spawn();

    if let Ok(mut child) = child {
        // write file to stdin if it is a fileBuffer
        if let PopplerFile::Buffer(file_buffer) = file {
            if let Some(mut stdin) = child.stdin.take() {
                let stdin_handle = thread::spawn(move || stdin.write_all(&file_buffer.buffer));
                // handle error if unable to write to stdin
                match stdin_handle.join() {
                    Ok(Ok(())) => {
                        // Everything is fine, continue with your code
                    }
                    Ok(Err(e)) => {
                        // stdin.write_all(&file_buffer.buffer) returned an error
                        eprintln!("Error: {:?}", e);
                        return Err(e);
                    }
                    Err(_) => {
                        // The child thread panicked
                        let e =
                            std::io::Error::new(std::io::ErrorKind::Other, "Child thread panicked");
                        eprintln!("Error: {:?}", e);
                        return Err(e);
                    }
                }
            }
        }

        // read stdout and stderr
        if let Ok(output) = child.wait_with_output() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

            if !stderr.is_empty() {
                eprintln!("Poppler Error: {}", &stderr);
            }

            if output.status.success() {
                println!("{}", &stdout);
                Ok(stdout)
            } else {
                // we already printed stderr, so just return a generic error
                Err(std::io::Error::new(std::io::ErrorKind::Other, stderr))
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to wait for output from child process",
            ))
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to spawn child process",
        ))
    }
}

fn get_path_to_executable(prog_name: &str) -> Result<PathBuf, std::io::Error> {
    // get the proper executable for the current operating system (ELF, Mach-O, PE)
    let os = std::env::consts::OS;

    // determine which of 3 executable folders to use
    let path_to_executable = match os {
        "windows" => format!("src/poppler/win/bin/{}.exe", prog_name),
        "macos" => format!("src/poppler/mac/bin/{}", prog_name),
        "ios" => format!("src/poppler/mac/bin/{}", prog_name),
        _ => format!("src/poppler/unix/bin/{}", prog_name),
    };
    // get absolute path of this root directory
    let root = std::env::current_dir();
    match root {
        Ok(root) => {
            let exe_path = root.join(path_to_executable);
            Ok(exe_path)
        }
        Err(e) => {
            // return error: no permission to access current directory
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
