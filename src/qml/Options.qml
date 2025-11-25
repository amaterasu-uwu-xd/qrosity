import QtQuick 2.15
import QtQuick.Controls 2.15
import org.kde.kirigami 2.19 as Kirigami
import QtQuick.Layouts 1.15

Kirigami.FormLayout {
    // Exponemos las propiedades para que main.qml las lea
    property alias eccLevel: eccCombo.currentText
    property alias quietZone: quietSlider.value

    ComboBox {
        id: eccCombo
        Kirigami.FormData.label: "Error Correction:"
        model: ["low", "medium", "quartile", "high"]
        currentIndex: 1 // Default M
    }

    RowLayout {
        Kirigami.FormData.label: "Margin:"
        Slider {
            id: quietSlider
            from: 0; to: 10; stepSize: 1
            value: 4
        }
        Label { text: quietSlider.value }
    }
}