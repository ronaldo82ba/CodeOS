mod handler;

mod installer;

mod manifest;

mod packager;

mod registry;

mod uninstaller;

mod validator;



pub use handler::{register_ipc_endpoint, register_ipc_endpoint_on, PkgService};

pub use installer::{CappInstaller, InstallError, InstallResult};

pub use manifest::{

    AppSection, CodeOsManifest, EntrySection, MetadataSection, PermissionsSection, UiSection,

};

pub use packager::{CappPackager, PackError};

pub use registry::{apps_dir, get_registry, install_dir_for, lookup_app, AppRegistry, InstalledApp};

pub use uninstaller::{CappUninstaller, UninstallError};

pub use validator::{CappValidator, ValidationError};


