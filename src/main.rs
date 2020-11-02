use std::fs;

fn main() {    
    env_logger::init();

    let dataset = "./census.csv";
    println!("Reading dataset from{}", dataset);
    let file = fs::File::open(dataset).expect("Cannot read dataset");
    let mut csv_reader = csv::Reader::from_reader(file);

    for record in csv_reader.records() {
        let record = record.expect("Invalid record");

        match record.get(1) {
            Some(population) => log::trace!("Population: {}", population),
            None => log::warn!("No population value!"),
        };
        // let city = record.get(0);
        // let population = record.get(1);

        // log::trace!("{:?} population: {:?}", city, population)
    }
}
