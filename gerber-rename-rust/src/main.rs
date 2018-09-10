use std::fs;
use std::path::PathBuf;
use std::env;

struct GerberExtension<'a> {
    from: &'a str,
    to: &'a str,
}

const GERBER_EXTENSIONS: [GerberExtension; 8] = [
    GerberExtension {
        from: "-B.SilkS.gbo",
        to: ".GBO",
    },
    GerberExtension {
        from: "-B.Mask.gbs",
        to: ".GBS",
    },
    GerberExtension {
        from: "-B.Cu.gbl",
        to: ".GBL",
    },
    GerberExtension {
        from: "-F.Cu.gtl",
        to: ".GTL",
    },
    GerberExtension {
        from: "-F.Mask.gts",
        to: ".GTS",
    },
    GerberExtension {
        from: "-F.SilkS.gto",
        to: ".GTO",
    },
    GerberExtension {
        from: "-Edge.Cuts.gm1",
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
    dir_entries.flat_map(|entry| {
        let file = entry.unwrap();
        let file_name = file.file_name().into_string().unwrap().clone();
        let file_name_copy = file_name.clone();

        GERBER_EXTENSIONS
            .iter()
            .filter(move |ext| file_name.contains(ext.from))
            .map(move |ext| {
                let new_file_name = file_name_copy.replace(ext.from, ext.to);
                let new_file_path = file.path().with_file_name(new_file_name);
                (file.path(), new_file_path)
            })
    }).for_each(|(old_path, new_path)| {
        match fs::rename(&old_path, &new_path) {
            Ok(_) => {
                let old_file_name = old_path.file_name().unwrap().to_str().unwrap();
                let new_file_name = new_path.file_name().unwrap().to_str().unwrap();
                println!("{} -> {}", old_file_name, new_file_name)
            }
            Err(err) => println!("Error: {}", err)
        };
    });
}
