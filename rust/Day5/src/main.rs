use std::fs::read_to_string;

#[derive(Debug)]
struct Range {
    destination: u64,
    source: u64,
    size: u64,
}

fn main() {
    println!("Hello, world!");
    let seeds = load_seeds();
    let seed_to_soil = load_map("seed-to-soil");
    let soil_to_fertilizer = load_map("soil-to-fertilizer");
    let fertilizer_to_water = load_map("fertilizer-to-water");
    let water_to_light = load_map("water-to-light");
    let light_to_temperature = load_map("light-to-temperature");
    let temperature_to_humidity = load_map("temperature-to-humidity");
    let humidity_to_location = load_map("humidity-to-location");

    let mut locations: Vec<u64> = vec![];
    for seed in seeds {
        println!("Compute for the seed {}", seed);
        let soil = get_destination(seed, &seed_to_soil);
        let fertilizer = get_destination(soil, &soil_to_fertilizer);
        let water = get_destination(fertilizer, &fertilizer_to_water);
        let light = get_destination(water, &water_to_light);
        let temperature = get_destination(light, &light_to_temperature);
        let humidity = get_destination(temperature, &temperature_to_humidity);
        let location = get_destination(humidity, &humidity_to_location);
        println!(
            "{}, {}, {}, {}, {}, {}, {}, {}",
            seed, soil, fertilizer, water, light, temperature, humidity, location
        );
        locations.push(location);
    }
    println!("Location: {:?}", locations.iter().min());

    let mut location_min: u64 = u64::MAX;
    let seeds2 = load_seeds();
    println!("Seeds2 {:?}", seeds2);
    for i in (0..seeds2.len()).step_by(2) {
        println!(
            "Compute the seed interval {} - {}",
            &seeds2[i],
            &seeds2[i] + &seeds2[i + 1]
        );

        for seed in seeds2[i]..seeds2[i] + seeds2[i + 1] {
            //println!("Compute for the seed {}", seed);
            let soil = get_destination(seed, &seed_to_soil);
            let fertilizer = get_destination(soil, &soil_to_fertilizer);
            let water = get_destination(fertilizer, &fertilizer_to_water);
            let light = get_destination(water, &water_to_light);
            let temperature = get_destination(light, &light_to_temperature);
            let humidity = get_destination(temperature, &temperature_to_humidity);
            let location = get_destination(humidity, &humidity_to_location);
            /*println!(
                "{}, {}, {}, {}, {}, {}, {}, {}",
                seed, soil, fertilizer, water, light, temperature, humidity, location
            );*/

            if location < location_min {
                location_min = location;
            }
        }
    }
    println!("Location: {:?}", location_min);
}

fn get_destination(number: u64, map: &Vec<Range>) -> u64 {
    for range in map {
        if number >= range.source && number < range.source + range.size {
            let out = range.destination + (number - range.source);
            //println!("Looking for {} in {:?} = {}", number, range, out);
            return out;
        }
    }
    //println!("Looking for {} No Range = {}", number, number);
    number
}

fn load_seeds() -> Vec<u64> {
    let lines: String = read_to_string("seeds").unwrap();
    lines.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn load_map(name: &str) -> Vec<Range> {
    let mut out: Vec<Range> = vec![];
    for line in read_to_string(name).unwrap().lines() {
        let vec: Vec<u64> = line.split(' ').map(|s| s.parse().unwrap()).collect();
        let range = Range {
            destination: vec[0],
            source: vec[1],
            size: vec[2],
        };
        out.push(range);
    }
    out
}
