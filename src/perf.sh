loops=0
total=0
lasttime=`curl -s $1/_cat/allocation | awk '{sum+=substr($2,1,length($2)-2)} END {print sum}'`
while true
do
    thistime=`curl -s $1/_cat/allocation | awk '{sum+=substr($2,1,length($2)-2)} END {print sum}'`
    # echo $thistime
    # echo $lasttime
    diff=$(echo "($thistime-$lasttime)"| bc -l)
    echo "last min: $diff"
    lasttime=$thistime
    loops=$(echo "($loops + 1)" | bc -l)
    total=$(echo "($total + $diff)" | bc -l)
    speed=$(echo "(($total / $loops) * 6)" | bc -l )
    echo "average since start $speed GB/min"
    sleep 10
done    