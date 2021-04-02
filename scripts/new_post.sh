cd public/post
today=$(date +"%Y-%m-%d")
file_name=$today-"$@"".md"
echo $file_name
cat << EOF > $file_name
---
title:
category:
tags: []
summary:
date: 令和
---

EOF
