#!/bin/sh
die () {
    echo >&2 "$@"
    exit 1
}

HELP="init_day.sh <day_number>"

# arg parse and validation
[ "$#" -eq 1 ] || die $HELP
[[ "$1" =~ ^[0-9]+$ ]] || die "$HELP : must be a number"
DAY_NUM=$1

read -p "Initializing Day $DAY_NUM [y|n]? "
echo ""
[[ $REPLY =~ ^[Yy]$ ]] || exit 0

DIR="day$DAY_NUM"
mkdir -p -- $DIR

cat > "$DIR/mod.rs" <<- EOM
pub mod part1;
pub mod part2;
EOM

cat > "$DIR/part1.rs" <<- EOM
pub fn main() {
    println!("Day $DAY_NUM Part 1");
}
EOM
cat > "$DIR/part2.rs" <<- EOM
pub fn main() {
    println!("Day $DAY_NUM Part 2");
}
EOM

echo "mod $DIR;\n" | cat - main.rs > temp && mv temp main.rs

echo "Day $DAY_NUM initialized"
