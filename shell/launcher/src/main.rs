#[cfg(target_os = "linux")]
fn main() -> std::process::ExitCode {
    use std::{env, process::ExitCode};
    use sunrise_app_menu::catalog;

    let mut args = env::args_os().skip(1);
    let command = args.next();
    let desktop_id = args.next();
    if args.next().is_none() {
        match (command.as_deref().and_then(|arg| arg.to_str()), desktop_id) {
            (Some("list"), None) => {
                for app in catalog::discover() {
                    println!("{}\t{}", app.id(), app.name());
                }
                return ExitCode::SUCCESS;
            }
            (Some("launch"), Some(id)) => {
                let result = catalog::discover()
                    .into_iter()
                    .find(|app| std::ffi::OsStr::new(app.id()) == id)
                    .ok_or_else(|| "No visible application has that desktop ID.".to_owned())
                    .and_then(|app| app.launch().map_err(|error| error.to_string()));
                return match result {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(error) => {
                        eprintln!("Launch failed: {error}");
                        ExitCode::FAILURE
                    }
                };
            }
            _ => {}
        }
    }
    eprintln!("Usage: sunrise-app-menu list | launch <desktop-id>");
    ExitCode::from(2)
}

#[cfg(not(target_os = "linux"))]
fn main() -> std::process::ExitCode {
    eprintln!("Sunrise App Menu requires a Linux desktop session.");
    std::process::ExitCode::FAILURE
}
