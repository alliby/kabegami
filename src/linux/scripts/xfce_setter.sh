for i in $(xfconf-query -c xfce4-desktop -p /backdrop -l | grep -E -e "screen.*/monitor.*image-path$" -e "screen.*/monitor.*/last-image$"); do
    xfconf-query -c xfce4-desktop -p "$i" -n -t string -s ""
    xfconf-query -c xfce4-desktop -p "$i" -s ""
    xfconf-query -c xfce4-desktop -p "$i" -s "$1"
done
