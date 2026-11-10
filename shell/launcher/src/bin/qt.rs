#![cfg_attr(not(target_os = "linux"), allow(unused_imports))]
// The bridge must live at this binary's crate root: cxx-qt generates its C++
// from the same file in build.rs, and a nested module would produce a
// different token stream than the top-level parse. See the launcher README.
// The bridge lives in the library (src/qtbridge.rs); this binary only hosts
// the Qt application bootstrap.
#[cfg(target_os = "linux")]
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
#[cfg(target_os = "linux")]
fn main() -> std::process::ExitCode {
    use std::{
        process::ExitCode,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
    };

    // Referencing the bridge's Rust side ensures the linker extracts the
    // object member that provides `cxxbridge1$...` symbols the generated C++
    // needs; Qt-side module registration during QML import depends on them.
    {
        let _prewarmed = <sunrise_app_menu::qtbridge::AppCatalogRust as Default>::default();
        std::mem::forget(_prewarmed);
    }

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    let (Some(app), Some(mut engine)) = (app.as_mut(), engine.as_mut()) else {
        eprintln!("Could not initialize Qt.");
        return ExitCode::FAILURE;
    };

    let failed = Arc::new(AtomicBool::new(false));
    let failure_flag = Arc::clone(&failed);
    let created = Arc::new(AtomicBool::new(false));
    let created_flag = Arc::clone(&created);
    // Two connections: object_created confirms the root object, and
    // object_creation_failed captures the URL that failed to load. QML
    // engine warnings go through Qt's logging, so keep both for evidence.
    let _c_ok = engine.as_mut().on_object_created(move |_, qobject, url| {
        eprintln!(
            "object_created: {} (object null: {})",
            url.to_string().as_str(),
            qobject.is_null()
        );
        created_flag.store(!qobject.is_null(), Ordering::Relaxed);
    });
    let _c_fail = engine.as_mut().on_object_creation_failed(move |_, url| {
        eprintln!("QML object creation failed: {}", url.to_string().as_str());
        failure_flag.store(true, Ordering::Relaxed);
    });
    engine
        .as_mut()
        .load(&QUrl::from("qrc:/qt/qml/Sunrise/AppMenu/qml/Main.qml"));
    if failed.load(Ordering::Relaxed) || !created.load(Ordering::Relaxed) {
        eprintln!("Could not load the application menu.");
        return ExitCode::FAILURE;
    }
    if app.exec() == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(not(target_os = "linux"))]
fn main() -> std::process::ExitCode {
    eprintln!("The Sunrise Qt probe requires Linux.");
    std::process::ExitCode::FAILURE
}
