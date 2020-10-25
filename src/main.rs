extern crate glob;

use std::collections::{HashMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Write;
use std::iter::FromIterator;

#[derive(Debug)]
struct OutputFile {
    file: fs::File,
    nb_chars: usize,
    nb_ranges: usize,
}

fn parse_escape(s: &str) -> String {
    if s == "none" {
        "".to_owned()
    } else {
        s
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
    }
}

fn main() -> Result<(), ()> {
    let args: Vec<_> = env::args().collect();

    let input_file_path = args.get(1).unwrap_or_else(|| panic!("Usage: {} <script_file>", args[0]));

    let mut total_nb_warnings: usize = 0;

    let mut output_file_path = None;
    let mut range_prefix = "".to_owned();
    let mut range_suffix = "\n\n".to_owned();
    let mut enable_range_headers = true;
    let mut enable_overall_stats = true;

    let mut files = HashMap::new();
    let mut chars_filter: Option<BTreeSet<char>> = None;

    let cfg = fs::read_to_string(input_file_path).unwrap_or_else(|e| panic!("Failed to read `{}`: {}", input_file_path, e));
    for mut line in cfg.lines() {
        line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        println!("Parsing line: `{}`", line);

        let mut keyval = line.splitn(2, ":");
        let lhs = keyval.next().expect("Expected left-hand side before `:`").trim();
        let rhs = keyval.next().expect("Expected right-hand side after `:`").trim();

        if !lhs.starts_with("U+") {
            match lhs {
                "output_file_path" => {
                    let file = fs::File::create(rhs).unwrap_or_else(|e| panic!("Failed to open output file `{}`: {}", rhs, e));
                    files.entry(rhs.to_owned()).or_insert(OutputFile {
                        file,
                        nb_chars: 0,
                        nb_ranges: 0,
                    });
                    output_file_path = Some(rhs.to_owned());
                },
                "range_prefix" => range_prefix = parse_escape(&rhs),
                "range_suffix" => range_suffix = parse_escape(&rhs),
                "range_headers" => {
                    enable_range_headers = match rhs {
                        "all" => true,
                        "none" => false,
                        unknown => panic!("Unknown value for `{}`: `{}`. Expected `all` or `none`", lhs, unknown),
                    }
                },
                "overall_stats" => {
                    enable_overall_stats = match rhs {
                        "all" => true,
                        "none" => false,
                        unknown => panic!("Unknown value for `{}`: `{}`. Expected `all` or `none`", lhs, unknown),
                    }
                },
                "filter_clear" => {
                    chars_filter = None;
                },
                "filter_add_files" => {
                    // Sync all files, in case they are referenced in the command. Yes, this is a supported use case.
                    for outfile in files.values() {
                        outfile.file.sync_all().expect("sync_all() failed");
                    }

                    for pat in rhs.split_whitespace() {
                        println!("{}: Expanding `{}`", lhs, pat);
                        let matches = glob::glob(pat).expect("Failed to expand wildcards");
                        for m in matches {
                            let path = m.unwrap();
                            println!("{}: Executing on `{}`", lhs, path.display());
                            let contents = fs::read_to_string(&path).expect("Failed to read file");
                            if chars_filter.is_none() {
                                chars_filter = Some(Default::default());
                            }
                            chars_filter.as_mut().unwrap().append(&mut contents.chars().collect());
                        }
                    }
                },
                "filter_dump" => {
                    if let Some(chars_filter) = chars_filter.as_ref() {
                        if files.contains_key(rhs) {
                            panic!("`{}`: File `{}` is already opened (this is probably a mistake?); aborting.", lhs, rhs);
                        } else {
                            fs::write(&rhs, String::from_iter(chars_filter.iter().cloned())).expect("Failed to write to file");
                        }
                    } else {
                        panic!("{}: No filter to dump", lhs);
                    }
                },
                unknown => panic!("Unknown command: `{}`", unknown),
            };

            continue;
        }

        let mut range_tokens = lhs.split_whitespace();
        let range_first_token = range_tokens.next().expect("Invalid range: missing range start").trim();
        let range_ellipsis_token = range_tokens.next().expect("Invalid range: missing ellipsis").trim();
        let range_last_token = range_tokens.next().expect("Invalid range: missing range end").trim();

        assert!(range_first_token.starts_with("U+"));
        assert_eq!(range_ellipsis_token, "...");
        assert!(range_last_token.starts_with("U+"));

        let first_codepoint = u32::from_str_radix(&range_first_token[2..], 16).expect("Failed to parse first codepoint");
        let last_codepoint = u32::from_str_radix(&range_last_token[2..], 16).expect("Failed to parse first codepoint");

        let nb_chars_predicted = (1 + last_codepoint - first_codepoint) as usize;
        let mut nb_chars_added_in_range = 0;
        let mut all_chars_in_range = String::with_capacity(nb_chars_predicted);
        let mut nb_warnings_for_range: usize = 0;
        for codepoint in first_codepoint .. last_codepoint + 1 {
            if let Some(c) = std::char::from_u32(codepoint) {
                let should_add = match chars_filter.as_ref() {
                    Some(chars_filter) => chars_filter.contains(&c),
                    None => true,
                };
                if should_add {
                    all_chars_in_range.push(c);
                    nb_chars_added_in_range += 1;
                }
            } else {
                eprintln!("WARNING: Codepoint U+{:X} is not a valid Rust char", codepoint);
                nb_warnings_for_range += 1;
            }
        }

        let mut output = String::new();

        if nb_chars_added_in_range >= 1 {
            if enable_range_headers {
                output += &format!("# Range: {} ({} chars)\n", lhs, nb_chars_added_in_range);
                if nb_warnings_for_range >= 1 {
                    output += &format!("# Warnings: {}\n", nb_warnings_for_range);
                }
                output += &format!("# Name: {}\n", rhs);
            }
            
            output += &format!("{}{}{}", range_prefix, all_chars_in_range, range_suffix);

            let file_id = output_file_path.as_ref().expect("output_file_path was not specified");
            let outfile = files.get_mut(file_id).unwrap();
            write!(outfile.file, "{}", output).expect("Failed to write to file");
            outfile.nb_chars += nb_chars_added_in_range;
            outfile.nb_ranges += 1;
        }

        total_nb_warnings += nb_warnings_for_range;
    }

    if enable_overall_stats {
        for outfile in files.values() {
            write!(&outfile.file, "# Overall stats: {} ranges, {} chars\n", outfile.nb_ranges, outfile.nb_chars).expect("Failed to write to file");
        }
    }

    if total_nb_warnings >= 1 {
        println!("Finished with {} warnings.", total_nb_warnings);
    } else {
        println!("Finished.");
    }

    Ok(())
}
