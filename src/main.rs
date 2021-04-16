use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    /// The file to summarize
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,

    /// Commands used to mark implications (opening:closing,opening:closing,...), can be used, multiple times.
    /// Use "@envname" instead of "opening:closing" to specify an environment.
    /// Do not include the backslash.
    implications_markers: String,
}

#[derive(Debug)]
struct Marker {
    begin: String,
    end: String,
}

impl Marker {
    fn from_environment(environment_name: &str) -> Marker {
        Marker {
            begin: String::from(r#"\begin{"#.to_owned() + environment_name + "}" ),
            end: String::from(r#"\end{"#.to_owned() + environment_name + "}" ),
        }
    }

    fn from_string(spec: &str) -> Marker {
        if spec.starts_with('@') {
            Marker::from_environment(&spec[1..])
        } else {
            let halves = spec.split(":").collect::<Vec<&str>>();
            Marker{
                begin:  format!("\\{}", String::from(halves[0]))  ,
                end:    format!("\\{}", String::from(halves[1]))  ,
            }
        }
    }
}

impl PartialEq for Marker {
    fn eq(&self, other: &Self) -> bool {
        self.begin == other.begin && self.end == other.end
    }
}

fn main() {
    let args = Cli::from_args();
    // Parse implication markers
    let implications_markers = &args.implications_markers
        .split(",")
        .map(Marker::from_string)
        .collect::<Vec<Marker>>();
    // println!("{:#?}", implications_markers);

    // Read from file, remove comments
    let lines = std::fs::read_to_string(&args.file)
        .expect(&format!("Couldn't read file {:?}", &args.file)[..]);
    let lines = lines.lines()
        .filter(|&line| !line.starts_with('%'));

    // Declare variables
    let mut implications: Vec<String> = vec![];
    let mut current_chunk: Vec<String> = vec!();
    let mut current_marker: Option<&Marker> = None;

    // Prepend with header
    for line in lines.clone() {
        if line.starts_with("\\section{") {
            break;
        }
        implications.append(&mut vec![ String::from(line)]);
    }

    // Collect all chunks
    for line in lines {
        if line.starts_with("\\section") || line.starts_with("\\subsection") || line.starts_with("\\newcommand") {
            implications.append(&mut vec![String::from(line)]);
            continue;
        }
        current_chunk.append(&mut vec![ String::from(line) ]);
        if current_marker.is_none() {
            match implications_markers.iter().find(|&m| line.starts_with(&m.begin)) {
                Some(marker) => {
                    current_marker = Some(marker);
                }
                None => {
                    current_chunk.clear();
                }
            }
        } else {
            match implications_markers.iter().find(|&m| line.starts_with(&m.end) && m == current_marker.unwrap()) {
                Some(_) => {
                    current_marker = None;
                    implications.append(&mut current_chunk);
                    current_chunk.clear();
                }
                None => {
                }
            }
        }
    }
    println!("{}\n\\end{{document}}", implications.join("\n"));
}

