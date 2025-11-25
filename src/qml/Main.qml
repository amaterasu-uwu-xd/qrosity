import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import org.kde.kirigami 2.19 as Kirigami
import "./forms" // Importamos la carpeta de formularios

Kirigami.ApplicationWindow {
    id: root
    width: 450
    height: 700
    title: "Qrosity"

    pageStack.initialPage: Kirigami.Page {
        // Remove title space
        title: ""
        header: null

        RowLayout {
            anchors.fill: parent
            spacing: 0

            ScrollView {
                id: leftPanel
                Layout.preferredWidth: parent.width * 0.4
                Layout.fillHeight: true
                Layout.alignment: Qt.AlignTop

                clip: true

                ColumnLayout {
                    width: leftPanel.availableWidth
                    spacing: Kirigami.Units.largeSpacing

                    Kirigami.FormLayout {
                        Layout.fillWidth: true
                        
                        ComboBox {
                            id: typeSelector
                            Kirigami.FormData.label: "QR Type:"
                            model: ["Text", "WiFi", "Contact"]
                            currentIndex: 0
                        }
                    }

                    Kirigami.Separator { Layout.fillWidth: true }

                    Loader {
                        id: formLoader
                        Layout.fillWidth: true
                        Layout.leftMargin: Kirigami.Units.largeSpacing
                        Layout.rightMargin: Kirigami.Units.largeSpacing

                        // Lógica de cambio de formulario
                        sourceComponent: {
                            switch (typeSelector.currentIndex) {
                                case 0: return textComponent;
                                case 1: return wifiComponent;
                                case 2: return contactComponent;
                            }
                        }
                    }

                    Kirigami.Separator { Layout.fillWidth: true }

                    // --- SECCIÓN 3: OPCIONES FIJAS ---
                    // Un componente reutilizable para ECC, Colores, etc.
                    Options {
                        id: globalOptions
                        Layout.leftMargin: Kirigami.Units.largeSpacing
                        Layout.rightMargin: Kirigami.Units.largeSpacing
                        Layout.fillWidth: true
                    }

                    // --- BOTÓN DE ACCIÓN ---
                    Button {
                        text: "Generate QR"
                        Layout.topMargin: Kirigami.Units.largeSpacing
                        Layout.leftMargin: Kirigami.Units.largeSpacing
                        Layout.alignment: Qt.AlignCenter
                        
                        onClicked: {
                            // Lógica para recolectar datos y enviarlos a Rust
                            // formLoader.item accede al formulario cargado actualmente
                            console.log("Generando tipo:", typeSelector.currentText)
                            console.log("Datos:", formLoader.item.getData())
                            console.log("ECC:", globalOptions.eccLevel)
                        }
                    }
                }
            }

            Kirigami.Separator {
                Layout.fillHeight: true
                Layout.preferredWidth: Kirigami.Units.smallSpacing
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                // Usamos el color de fondo del tema, o un gris suave para resaltar el "papel"
                color: Kirigami.Theme.backgroundColor 

                // Un contenedor "carta" para el QR
                Rectangle {
                    id: qrCard
                    width: Math.min(parent.width, parent.height) * 0.9
                    height: width
                    anchors.centerIn: parent
                    
                    radius: 10
                    
                    // Sombra para efecto de elevación (opcional)
                    layer.enabled: true
                    layer.effect: Kirigami.ShadowedRectangle {
                        radius: 10
                        shadow.size: 15
                        shadow.color: Qt.rgba(0,0,0,0.2)
                    }

                    // La Imagen del QR
                    Image {
                        id: qrPreview
                        anchors.fill: parent
                        anchors.margins: 20 // Un poco de aire dentro de la tarjeta
                        fillMode: Image.PreserveAspectFit
                        smooth: false // false es mejor para QR (píxeles nítidos)
                        
                        // Placeholder si no hay imagen aún
                        source: "" 
                    }

                    // Texto de ayuda si no hay imagen
                    Label {
                        anchors.centerIn: parent
                        text: "Tu código aparecerá aquí"
                        visible: qrPreview.status !== Image.Ready
                        opacity: 0.5
                    }
                }
            }
        }
    }

    // --- DEFINICIÓN DE COMPONENTES ---
    // Definimos qué archivo cargar para cada caso
    Component { id: wifiComponent; WifiForm { } }
}