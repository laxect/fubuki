curl https://pubsubhubbub.appspot.com/ --header "Content-Type: application/x-www-form-urlencoded" -d "hub.mode=publish&&hub.url=https://blog.gyara.moe/atom.xml" -v
echo "


======


"
curl "https://webmention.app/check/?url=https://blog.gyara.moe/atom.xml" -X POST -v
