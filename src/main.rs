use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    first_path: std::path::PathBuf,
    second_path: std::path::PathBuf,
}

fn file_to_vec(path: &std::path::PathBuf) -> Result<Vec<csv::StringRecord>, csv::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(path)
        .unwrap();

    Ok(reader
        .records()
        .filter_map(|res| res.ok())
        // .map(|row| row.get(1).unwrap().into())
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!(
        "First file: {}\nSecond file: {}",
        args.first_path.display(),
        args.second_path.display()
    );

    let mut longer = file_to_vec(&args.first_path).unwrap(); // This is unibet
    let mut shorter = file_to_vec(&args.second_path).unwrap(); // This is bet365

    if longer.len() < shorter.len() {
        (longer, shorter) = (shorter, longer);
    }

    let name = "combined.csv";

    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(&name)
        .unwrap();

    writer
        .write_record(vec!["Erbjudande", "unibet", "bet365"])
        .unwrap();

    writer.flush().unwrap();

    let mut longer_index = 0;
    for short in shorter.into_iter() {
        let short_label = &short[0]; // IDK if trim is necessary
        let mut long_label = &longer[longer_index][0];
        while short_label != long_label {
            longer_index += 1;
            long_label = &longer[longer_index][0].trim();
        }

        writer
            .write_record(vec![
                short[0].to_string(),
                short[1].to_string(),
                longer[longer_index][1].to_string(),
            ])
            .unwrap();
    }

    writer.flush().unwrap();
    println!("Wrote combined results to: {}", &name);

    Ok(())
}
