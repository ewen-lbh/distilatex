use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    /// The file to summarize
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,

    /// Commands used to mark implications (opening:closing,opening:closing,...), can be used, multiple times.
    /// Use "@envname" instead of "opening:closing" to specify an environment.
    /// Do not include the backslash.
    markers: String,
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
    let markers = &args.markers
        .split(",")
        .map(Marker::from_string)
        .collect::<Vec<Marker>>();

    // Read from file, remove comments
    let lines = std::fs::read_to_string(&args.file)
        .expect(&format!("Couldn't read file {:?}", &args.file)[..]);
    let lines = lines.lines()
        .filter(|&line| !line.starts_with('%'));

    // Declare variables
    let mut summary: Vec<String> = vec![];
    let mut current_chunk: Vec<String> = vec!();
    let mut current_marker: Option<&Marker> = None;
    let mut header_ends_at: usize = 0;

    // Prepend with header
    for (i, line) in lines.clone().enumerate() {
        if line.starts_with("\\section{") {
            header_ends_at = i;
            break;
        }
        summary.append(&mut vec![ String::from(line)]);
    }

    // Collect all chunks
    for (i, line) in lines.enumerate() {
        // Don't include stuff twice, lines with indexes before header_ends_at are included anyway.
        // This prevents double declaration of \newcommand's for example.
        if i < header_ends_at {
            continue;
        }
        // Keep sections, subsections for structure
        // Keep (re)new{command,environment} commands to not fail to compile
        if line.starts_with("\\section") || line.starts_with("\\subsection") || line.starts_with("\\newcommand") || line.starts_with("\\renewcommand") || line.starts_with("\\newenvironment") || line.starts_with("\\renewenvironment") {
            summary.append(&mut vec![String::from(line)]);
            continue;
        }

        current_chunk.append(&mut vec![ String::from(line) ]);

        if current_marker.is_none() {
            match markers.iter().find(|&m| line.starts_with(&m.begin)) {
                Some(marker) => {
                    current_marker = Some(marker);
                }
                None => {
                    current_chunk.clear();
                }
            }
        } else {
            match markers.iter().find(|&m| line.starts_with(&m.end) && m == current_marker.unwrap()) {
                Some(_) => {
                    current_marker = None;
                    summary.append(&mut current_chunk);
                    current_chunk.clear();
                }
                None => {
                }
            }
        }
    }

    // Don't forget the \end{document} !!
    println!("{}\n\\end{{document}}", summary.join("\n"));
}

