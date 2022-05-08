#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub struct Sysbar(SysbarImpl);

impl Sysbar {
    pub fn new(name: &str) -> Self {
        Sysbar(SysbarImpl::new(name))
    }

    pub fn add_item(&mut self, label: &str, cbs: Box<dyn Fn()>) {
        self.0.add_item(label, cbs)
    }

    pub fn add_quit_item(&mut self, label: &str) {
        self.0.add_quit_item(label)
    }

    pub fn attach(&mut self) {
        self.0.attach()
    }

    pub fn detach() {
        SysbarImpl::detach()
    }

    pub fn show_window() {
        SysbarImpl::show_window()
    }

    pub fn hide_window() {
        SysbarImpl::hide_window()
    }
}

#[cfg(target_os = "macos")]
type SysbarImpl = macos::MacOsSysbar;

#[cfg(target_os = "linux")]
type SysbarImpl = linux::LinuxSysbar;
