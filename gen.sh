for i in $(seq 1 10);
do
    echo $i
    geng -q $i > "graphs${i}.txt"
    geng -qc $i > "cgraphs${i}.txt"
done
