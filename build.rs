use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    #[cfg(feature = "gui")]
    {
        CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "qrosity",
            rust_files: &["src/modes/gui.rs"],
            qml_files: &[
                "src/qml/Main.qml",
                "src/qml/Options.qml",
                "src/qml/forms/WifiForm.qml",
            ],
            ..Default::default()
        })
        .build();
    }
}