use gio::prelude::*;

pub struct Application {
    id: gio::glib::GString,
    info: gio::AppInfo,
}

impl Application {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> gio::glib::GString {
        self.info.display_name()
    }

    pub fn launch(&self) -> Result<(), gio::glib::Error> {
        self.info.launch(&[], gio::AppLaunchContext::NONE)
    }
}

pub fn discover() -> Vec<Application> {
    let mut apps: Vec<_> = gio::AppInfo::all()
        .into_iter()
        .filter_map(visible_application)
        .collect();
    apps.sort_by_cached_key(|app| (app.name().to_lowercase(), app.id().to_owned()));
    apps
}

fn visible_application(info: gio::AppInfo) -> Option<Application> {
    let desktop_entry = info.downcast_ref::<gio::DesktopAppInfo>()?;
    if desktop_entry.is_hidden() || !info.should_show() {
        return None;
    }
    Some(Application {
        id: info.id()?,
        info,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_entry(contents: &str) -> Option<gio::DesktopAppInfo> {
        let entry = gio::glib::KeyFile::new();
        entry
            .load_from_data(contents, gio::glib::KeyFileFlags::NONE)
            .unwrap();
        gio::DesktopAppInfo::from_keyfile(&entry)
    }

    #[test]
    fn invalid_desktop_entry_is_rejected() {
        assert!(
            load_entry(
                "[Desktop Entry]\nType=Link\nName=Invalid app\nURL=https://example.invalid\n"
            )
            .is_none()
        );
    }

    #[test]
    fn launch_surfaces_a_missing_working_directory() {
        let info = load_entry("[Desktop Entry]\nType=Application\nName=Test app\nExec=/bin/true\nPath=/proc/self/sunrise-missing-working-directory\n").unwrap();
        let app = Application {
            id: "test.desktop".into(),
            info: info.upcast(),
        };
        assert!(app.launch().is_err());
    }
}
