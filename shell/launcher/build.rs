fn main() {
    #[cfg(all(target_os = "linux", feature = "qt-probe"))]
    {
        // Cargo 1.98 drops `-l static=` directives from dependency build scripts;
        // re-emit the cxx-qt generated static libraries so their initializer
        // symbols resolve (shortcut: remove once cargo propagates these again,
        // or cxx-qt ships a build system that doesn't rely on dep directives).
        // Put the generated libs in a link group so ld.bfd resolves symbols
        // referenced by cxx-qt's whole-archive initializer objects regardless
        // of link-line order.
        println!("cargo:rustc-link-arg=-Wl,--start-group");
        println!("cargo:rustc-link-arg=-lsunrise-app-menu-cxxqt-generated");
        println!("cargo:rustc-link-arg=-lcxx-qt-cxxqt-generated");
        println!("cargo:rustc-link-arg=-lcxx-qt-lib-cxxqt-generated");
        println!("cargo:rustc-link-arg=-lQt6Core");
        println!("cargo:rustc-link-arg=-lQt6Gui");
        println!("cargo:rustc-link-arg=-lQt6Qml");
        println!("cargo:rustc-link-arg=-lQt6Quick");
        println!("cargo:rustc-link-arg=-lstdc++");
        println!("cargo:rustc-link-arg=-Wl,--end-group");
        cxx_qt_build::CxxQtBuilder::new_qml_module(
            cxx_qt_build::QmlModule::new("Sunrise.AppMenu").qml_file("qml/Main.qml"),
        )
        .qt_module("Quick")
        .files(["src/qtbridge.rs"])
        .build();
        // cxx-qt-build emits `-l static:+whole-archive=cxx-qt-call-init-*`
        // initializer objects; on cargo 1.98 those directives don't reach the
        // linker and re-emitting them is rejected as modifier overriding, so
        // link the archives directly by path as whole objects instead.
        let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR always set for build scripts");
        println!(
            "cargo:rustc-link-arg=-Wl,--whole-archive,{out_dir}/libcxx-qt-call-init-crate_sunrise_app_menu.a"
        );
        println!(
            "cargo:rustc-link-arg=-Wl,--whole-archive,{out_dir}/libcxx-qt-call-init-qml_module_Sunrise_AppMenu.a"
        );
    }
}
