for file in *.svg; do
	inkscape "$file" -o "${file%.svg}.png"
done
