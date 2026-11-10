import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Sunrise.AppMenu

ApplicationWindow {
    id: window
    width: 420
    height: 500
    visible: true
    title: qsTr("Sunrise App Menu — development probe")

    AppCatalog { id: catalog }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16

        Label {
            text: qsTr("Applications")
            font.pixelSize: 22
        }

        ListView {
            id: apps
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            focus: true
            model: catalog.count()
            ScrollBar.vertical: ScrollBar {}

            delegate: ItemDelegate {
                required property int index
                width: ListView.view.width
                text: catalog.name(index)
                highlighted: ListView.isCurrentItem
                onClicked: {
                    apps.currentIndex = index
                    apps.launchSelected()
                }
            }

            function launchSelected() {
                if (currentIndex < 0)
                    return
                const error = catalog.launch(currentIndex)
                status.text = error.length
                    ? qsTr("Launch failed: %1").arg(error)
                    : qsTr("Launch requested for %1.").arg(catalog.name(currentIndex))
            }

            Keys.onReturnPressed: launchSelected()
            Keys.onEnterPressed: launchSelected()
        }

        Label {
            id: status
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: apps.count ? qsTr("Select an application to open.") : qsTr("No visible desktop applications were found.")
        }
    }
}
