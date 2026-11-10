// The CXX-Qt bridge for the QML application-menu probe. Keep the bridge, its
// Rust struct, and the method implementations in one crate-root file so the
// macro expansion matches the top-level parse performed in build.rs. This
// module is only referenced under `all(target_os = "linux", feature = "qt-probe")`.
use crate::catalog::{self, Application};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type AppCatalog = super::AppCatalogRust;

        #[qinvokable]
        fn count(&self) -> i32;

        #[qinvokable]
        fn name(&self, index: i32) -> QString;

        #[qinvokable]
        fn launch(&self, index: i32) -> QString;
    }
}

pub struct AppCatalogRust {
    apps: Vec<Application>,
}

impl Default for AppCatalogRust {
    fn default() -> Self {
        Self {
            apps: catalog::discover(),
        }
    }
}

impl ffi::AppCatalog {
    fn count(&self) -> i32 {
        self.rust().apps.len().try_into().unwrap_or(i32::MAX)
    }

    fn name(&self, index: i32) -> QString {
        self.application(index)
            .map_or_else(QString::default, |app| QString::from(app.name().as_str()))
    }

    fn launch(&self, index: i32) -> QString {
        match self.application(index) {
            Some(app) => match app.launch() {
                Ok(()) => QString::default(),
                Err(error) => QString::from(error.to_string().as_str()),
            },
            None => QString::from("No application at that index."),
        }
    }

    fn application(&self, index: i32) -> Option<&Application> {
        self.rust().apps.get(usize::try_from(index).ok()?)
    }
}
