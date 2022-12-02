if command -v qdbus-qt5; then
    qdbus_command=qdbus-qt5
else
    qdbus_command=qdbus
fi

$qdbus_command org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript "
var allDesktops = desktops();
for (i=0; i < allDesktops.length; i++) {
    d = allDesktops[i];
    d.wallpaperPlugin = 'org.kde.image';
    d.currentConfigGroup = Array('Wallpaper', 'org.kde.image', 'General');
    d.writeConfig('Image', 'file://"$1"')
}
"
