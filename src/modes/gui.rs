#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        type DummyQObject = super::DummyRustStruct;
    }
}

#[derive(Default)]
pub struct DummyRustStruct;

/// Runs the GUI mode.
/// Initializes the Qt application and loads the QML interface.
pub fn run() {
    use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQuickStyle, QString, QUrl};
    use cxx_qt_lib_extras::QApplication;
    use std::env;
    let mut app = QApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // To associate the executable to the installed desktop file
    QGuiApplication::set_desktop_file_name(&QString::from("qrosity"));
    // To ensure the style is set correctly
    let style = env::var("QT_QUICK_CONTROLS_STYLE");
    if style.is_err() {
        QQuickStyle::set_style(&QString::from("org.kde.desktop"));
    }

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/qrosity/src/qml/Main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}