import QtQuick 2.15
import QtQuick.Controls 2.15
import org.kde.kirigami 2.19 as Kirigami
import QtQuick.Layouts 1.15

Kirigami.FormLayout {
    // Función pública para que main.qml saque los datos
    function getData() {
        return {
            ssid: ssidField.text,
            password: passField.text,
            hidden: hiddenCheck.checked,
            security: securityCombo.currentText
        }
    }

    TextField {
        id: ssidField
        Kirigami.FormData.label: "Network Name (SSID):"
    }

    ComboBox {
        id: securityCombo
        Kirigami.FormData.label: "Security:"
        model: ["WPA", "WEP", "No Password"]
    }

    TextField {
        id: passField
        Kirigami.FormData.label: "Password:"
        echoMode: TextInput.Password
        visible: securityCombo.currentText !== "No Password"
    }

    CheckBox {
        id: hiddenCheck
        text: "Hidden Network"
    }
}