while read n; do
    f=$((n/3-2))
    part1=$((part1+f));
    while [ $f -gt 0 ]; do
        part2=$((part2+f))
        f=$((f/3-2))
    done
done
echo $part1
echo $part2
