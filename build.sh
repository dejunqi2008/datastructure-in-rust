if [ $1 = "test" ]; then
    echo "$1 run test only"
    cargo test
elif [ $1 = "no-warn" ]; then
    echo " build but igniore all warning message"
    cargo rustc -- -Awarnings 
elif [ -z "$1" ]; then
    echo "--- full build --- "
    cargo build
fi
