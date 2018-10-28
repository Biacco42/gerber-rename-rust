use std::fs;
use std::path::PathBuf;
use std::env;

struct GerberExtension<'a> {
    from: &'a str,
    to: &'a str,
}

const GERBER_EXTENSIONS: [GerberExtension; 8] = [
    GerberExtension {
        from: "-F.Cu.gbr",
        to: ".GTL",
    },
    GerberExtension {
        from: "-B.Cu.gbr",
        to: ".GBL",
    },
    GerberExtension {
        from: "-F.Mask.gbr",
        to: ".GTS",
    },
    GerberExtension {
        from: "-B.Mask.gbr",
        to: ".GBS",
    },
    GerberExtension {
        from: "-F.SilkS.gbr",
        to: ".GTO",
    },
    GerberExtension {
        from: "-B.SilkS.gbr",
        to: ".GBO",
    },
    GerberExtension {
        from: "-Edge.Cuts.gbr",
        to: ".GML",
    },
    GerberExtension {
        from: ".drl",
        to: ".TXT",
    },
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let path;
    if args.len() > 1 {
        path = PathBuf::from(&args[1]);
    } else {
        println!("Enter dir path!");
        return;
    };

    if !path.is_dir() {
        println!("This is not dir or not found : {:?}", path);
        return;
    }

    println!("Open : {:?}\n", path);

    let dir_entries = fs::read_dir(path).unwrap();
    dir_entries.flat_map(|entry| -> Vec<(PathBuf, PathBuf)> {
        let file = entry.unwrap();
        let file_name = file.file_name().into_string().unwrap().clone();

        GERBER_EXTENSIONS
            .iter()
            .filter(|ext| file_name.contains(ext.from))
            .map(|ext| {
                let new_file_name = file_name.replace(ext.from, ext.to);
                let new_file_path = file.path().with_file_name(&new_file_name);

                (file.path(), new_file_path)
            }).collect()
    }).for_each(|(old_path, new_path)| {
        match fs::rename(&old_path, &new_path) {
            Ok(_) => {
                let old_file_name = old_path.file_name().unwrap().to_str().unwrap();
                let new_file_name = new_path.file_name().unwrap().to_str().unwrap();
                println!("{:?} -> {:?}", &old_file_name, &new_file_name)
            }
            Err(err) => println!("Error: {}", err)
        };
    });
}
