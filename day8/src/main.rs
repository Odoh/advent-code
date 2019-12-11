use advent::InputSnake;
use itertools::Itertools;

fn parse_into_layers(s: &str, wide: usize, tall: usize) -> Vec<String> {
    let chars_per_layer = wide * tall;
    s.chars()
        .chunks(chars_per_layer)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect()
}

fn merge_layers(layers: Vec<String>, wide: usize, tall: usize) -> String {
    let chars_per_layer = wide * tall;
    let mut merged = String::with_capacity(chars_per_layer);
    for i in 0..=chars_per_layer {
        for layer in layers.iter() {
            match layer.chars().nth(i) {
                Some('0') => {
                    merged.push('▓');
                    break;
                },
                Some('1') => {
                    merged.push('░');
                    break;
                }
                Some('2') | Some(_) | None => {},
            }
        }
    }
    merged
}

fn part_one() {
    let input = InputSnake::new("input");
    let layers = parse_into_layers(&input.no_snake(), 25, 6);

    let (fewest_zeros_layer, _) = layers.iter()
        .map(|layer| (layer, layer.chars().filter(|&c| c == '0').count()))
        .min_by_key(|&(_, count)| count)
        .unwrap();
    let ones = fewest_zeros_layer.chars().filter(|&c| c == '1').count();
    let twos = fewest_zeros_layer.chars().filter(|&c| c == '2').count();

    println!("Part One: {:?}", ones * twos);
}

fn part_two() {
    let input = InputSnake::new("input");
    let layers = parse_into_layers(&input.no_snake(), 25, 6);

    let merged = merge_layers(layers, 25, 6);
    let merged_lines = merged.chars()
            .chunks(25)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .collect::<Vec<String>>();

    println!("Part Two:");
    for line in merged_lines {
        println!("{}", line);
    }
}

fn main() {
    part_one();
    part_two();
}
