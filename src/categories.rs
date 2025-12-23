use std::{collections::HashMap, ffi::OsStr};


pub fn category_extension_map() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        // 🎬 Videos
        (
            "Videos",
            vec![
                "mp4", "mov", "avi", "mkv", "flv", "wmv", "webm", "m4v", "mpg", "mpeg", "3gp",
                "3g2", "h264", "h265", "hevc", "av1", "vp9", "m2ts", "mts", "vob", "ogv", "ogm",
                "gifv", "mxf", "roq", "nsv", "f4v", "f4p", "f4a", "f4b", "asf", "rmvb", "divx",
                "xvid", "dv", "dif", "mjpeg", "mjp", "amv", "bik", "smv", "svi", "wtv", "yuv",
                "ivf", "drc",
            ],
        ),
        // 🖼️ Images / Raster
        (
            "Images",
            vec![
                "jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "tif", "tiff", "jfif",
                "jpe", "jif", "jfi", "dib", "psd", "xcf", "kra", "ora", "raw", "cr2", "nef", "orf",
                "sr2", "arw", "dng", "heic", "heif", "avif", "apng", "dds", "hdr", "exr", "pbm",
                "pgm", "ppm", "pnm", "sgi", "fits", "ras", "cut", "pic", "icns", "jp2", "j2k",
                "pfm", "qoi", "pam", "xpm", "xbm", "cur", "jxl", "jxr", "wdp",
            ],
        ),
        // 📄 Documents / Text
        (
            "Documents",
            vec![
                "pdf", "doc", "docx", "txt", "md", "markdown", "rtf", "odt", "odm", "fodt", "sxw",
                "tex", "latex", "wps", "wpd", "pages", "msg", "rst", "adoc", "dot", "dotx", "xps",
                "oxps", "chm", "djvu", "abw", "lwp", "sdw", "bib", "ris", "enw", "nfo", "readme",
            ],
        ),
        // 🎵 Audio
        (
            "Audio",
            vec![
                "mp3", "wav", "flac", "aac", "ogg", "oga", "opus", "m4a", "m4b", "m4p", "alac",
                "wma", "ape", "ac3", "dts", "tta", "aiff", "aif", "aifc", "caf", "mid", "midi",
                "mpa", "ra", "rm", "amr", "awb", "au", "wv", "spx", "gsm", "it", "xm", "s3m",
                "mod", "dsf", "dff", "dss", "voc", "vox", "mka", "8svx", "tak", "shn", "ofr",
                "ofs",
            ],
        ),
        // 📦 Archives / Compression
        (
            "Archives",
            vec![
                "zip", "zipx", "rar", "7z", "tar", "gz", "bz2", "xz", "z", "lz", "lzma", "lrz",
                "zst", "br", "tgz", "tbz", "tbz2", "tb2", "taz", "tz2", "tlz", "txz", "jar", "war",
                "ear", "apk", "deb", "rpm", "cab", "arj", "lzh", "ace", "dmg", "pkg", "sit",
                "sitx", "sea", "alz", "sfark", "cpio", "pak", "r00", "001", "wim", "esd", "lzip",
                "lz4", "snappy",
            ],
        ),
        // 💻 Code / Source Files
        (
            "Code",
            vec![
                "rs", "py", "js", "ts", "jsx", "tsx", "java", "cpp", "c", "h", "hpp", "hxx", "cxx",
                "cc", "cs", "go", "rb", "php", "swift", "kt", "kts", "scala", "pl", "sh", "bash",
                "zsh", "fish", "ps1", "lua", "r", "m", "mm", "html", "htm", "css", "scss", "sass",
                "less", "xml", "json", "yaml", "yml", "toml", "ini", "cfg", "conf", "vue",
                "svelte", "dart", "elm", "ex", "exs", "erl", "hrl", "hs", "lhs", "clj", "cljs",
                "cljc", "edn", "rkt", "scm", "sql", "vb", "vbs", "asm", "s", "nasm", "f", "f90",
                "f95", "for", "ftn", "nim", "ml", "mli", "fs", "fsi", "fsx", "fsscript", "v", "sv",
                "vhd", "vhdl", "zig", "vala", "pony", "odin", "mojo", "red", "ring", "io", "cr",
                "hack", "sol", "vy", "move", "wasm", "wat", "ll", "proto", "capnp", "thrift", "d",
                "nim", "carbon", "pkl", "groovy", "gradle", "rake", "makefile",
            ],
        ),
        // ⚙️ Executables / Binaries
        (
            "Executables",
            vec![
                "exe",
                "msi",
                "app",
                "appimage",
                "run",
                "bin",
                "elf",
                "out",
                "dll",
                "so",
                "dylib",
                "a",
                "o",
                "sys",
                "ko",
                "bat",
                "cmd",
                "com",
                "jar",
                "class",
                "dex",
                "ipa",
                "xap",
                "snap",
                "flatpak",
                "appx",
                "msix",
                "workflow",
                "action",
                "application",
            ],
        ),
        // 📊 Spreadsheets
        (
            "Spreadsheets",
            vec![
                "xlsx", "xls", "xlsm", "xlsb", "xltx", "xlt", "csv", "tsv", "tab", "ods", "fods",
                "sxc", "numbers", "dif", "dbf", "prn", "qpw", "gnumeric", "wk1", "wk3", "wk4",
                "wk5", "wks", "wb1", "wb2", "wb3", "123", "slk", "sylk",
            ],
        ),
        // 📽️ Presentations
        (
            "Presentations",
            vec![
                "pptx", "ppt", "pptm", "ppsx", "pps", "ppsm", "potx", "pot", "potm", "odp", "fodp",
                "key", "sxi", "sti", "sdd", "sdp", "shw", "show",
            ],
        ),
        // 🔤 Fonts
        (
            "Fonts",
            vec![
                "ttf", "otf", "ttc", "woff", "woff2", "woffz", "eot", "fon", "fnt", "pfb", "pfm",
                "pfa", "afm", "dfont", "suit", "bdf", "pcf", "sfd", "glif", "gxf",
            ],
        ),
        // 🧱 3D / Game / Graphics
        (
            "3D",
            vec![
                "obj",
                "fbx",
                "dae",
                "3ds",
                "blend",
                "stl",
                "ply",
                "gltf",
                "glb",
                "usd",
                "usda",
                "usdc",
                "usdz",
                "abc",
                "c4d",
                "max",
                "mb",
                "ma",
                "x3d",
                "wrl",
                "vrml",
                "lwo",
                "lws",
                "mdl",
                "mesh",
                "vrm",
                "pk3",
                "pak",
                "bsp",
                "wad",
                "uasset",
                "upk",
                "unitypackage",
                "prefab",
            ],
        ),
        // 🗄️ Databases
        (
            "Databases",
            vec![
                "db", "sqlite", "sqlite3", "mdb", "accdb", "sql", "dbf", "pdb", "mdf", "ldf",
                "ndf", "frm", "ibd", "realm", "rocksdb", "leveldb", "sled", "bson", "cdb", "tdb",
                "kdbx", "db3",
            ],
        ),
        // 📚 Ebooks / Comics
        (
            "Ebooks",
            vec![
                "epub", "mobi", "azw", "azw3", "azw4", "kf8", "kfx", "fb2", "djvu", "cbr", "cbz",
                "cbt", "cb7", "cba", "lit", "lrf", "prc", "pdb", "opf", "ncx", "ibooks", "zbf",
                "rb", "pdg",
            ],
        ),
        // 📐 Vector / Design
        (
            "Vector",
            vec![
                "svg", "ai", "eps", "cdr", "cgm", "wmf", "emf", "sk", "sk1", "plt", "hpgl", "dxf",
                "dwg", "fig", "xd", "sketch", "afdesign", "afphoto", "afpub", "vsd", "vsdx",
            ],
        ),
        // 🏗️ CAD / Engineering
        (
            "CAD",
            vec![
                "dwg",
                "dxf",
                "dwf",
                "dgn",
                "rvt",
                "rfa",
                "ifc",
                "step",
                "stp",
                "iges",
                "igs",
                "sat",
                "sldprt",
                "sldasm",
                "slddrw",
                "catpart",
                "catproduct",
                "prt",
                "asm",
                "ipt",
                "iam",
                "idw",
                "x_t",
                "x_b",
                "jt",
                "par",
                "psm",
                "sdxf",
            ],
        ),
        // 📜 Scripts
        (
            "Scripts",
            vec![
                "sh", "bash", "zsh", "fish", "ksh", "csh", "tcsh", "ps1", "psm1", "psd1", "bat",
                "cmd", "vbs", "js", "py", "rb", "pl", "lua", "tcl", "awk", "sed", "nu", "raku",
                "rexx", "expect", "gawk", "nawk", "ahk", "autoit",
            ],
        ),
        // ⚙️ Configuration / Build / DevOps
        (
            "Config",
            vec![
                "ini",
                "cfg",
                "conf",
                "config",
                "yaml",
                "yml",
                "toml",
                "json",
                "xml",
                "properties",
                "env",
                "editorconfig",
                "gitignore",
                "dockerignore",
                "npmignore",
                "eslintrc",
                "babelrc",
                "prettierrc",
                "makefile",
                "cmake",
                "cmakecache",
                "gradle",
                "pom",
                "lock",
                "cargo.lock",
                "npmrc",
                "yarnrc",
                "pnpm-lock",
                "dockerfile",
                "helm",
                "tf",
                "tfvars",
                "tfstate",
                "nomad",
                "ansible",
                "vagrantfile",
                "procfile",
            ],
        ),
        // 📊 Data / Serialization / ML
        (
            "Data",
            vec![
                "json", "jsonl", "ndjson", "xml", "yaml", "yml", "toml", "csv", "tsv", "msgpack",
                "avro", "parquet", "orc", "arrow", "feather", "protobuf", "thrift", "pickle",
                "joblib", "npy", "npz", "hdf5", "h5", "zarr", "mat", "sav", "sas7bdat", "onnx",
                "pb", "tfrecord", "rdf", "ttl", "nt", "jsonld",
            ],
        ),
        // 💿 Disk Images / VM / Firmware
        (
            "DiskImages",
            vec![
                "iso",
                "img",
                "bin",
                "cue",
                "mdf",
                "mds",
                "nrg",
                "ccd",
                "sub",
                "toast",
                "vcd",
                "dmg",
                "sparseimage",
                "cdr",
                "daa",
                "uif",
                "b6t",
                "rom",
                "fw",
                "uefi",
                "bios",
                "vhd",
                "vhdx",
                "vmdk",
                "vdi",
                "vbox",
                "qcow",
                "qcow2",
                "raw",
                "dd",
            ],
        ),
        // 🧹 Temporary / Backup / System Junk
        (
            "Temp",
            vec![
                "tmp",
                "temp",
                "bak",
                "backup",
                "old",
                "swp",
                "swo",
                "~",
                "cache",
                "crdownload",
                "download",
                "part",
                "partial",
                "lock",
                "pid",
                "chk",
                "dmp",
                "stackdump",
                "thumbs.db",
                "desktop.ini",
                ".DS_Store",
                "autosave",
                "recovery",
            ],
        ),
        // 🌐 Web Development Assets (NEW!)
        (
            "WebAssets",
            vec![
                "hbs",
                "handlebars",
                "mustache",
                "ejs",
                "pug",
                "jade",
                "twig",
                "map",
                "css.map",
                "js.map",
                "webmanifest",
                "appcache",
                "browserconfig",
                "humans.txt",
                "robots.txt",
                "htaccess",
                "htpasswd",
            ],
        ),
        // 🔐 Certificates & Security (NEW!)
        (
            "Certificates",
            vec![
                "pem",
                "crt",
                "cer",
                "der",
                "p7b",
                "p7c",
                "p12",
                "pfx",
                "key",
                "pub",
                "ppk",
                "csr",
                "p7s",
                "asc",
                "gpg",
                "sig",
                "pgp",
                "keystore",
                "jks",
                "truststore",
            ],
        ),
        // 📋 Logs (NEW!)
        (
            "Logs",
            vec![
                "log", "logs", "out", "err", "trace", "journal", "syslog", "dmesg", "crash",
                "dump", "core",
            ],
        ),
        // 📦 Packages & Dependencies (NEW!)
        (
            "Packages",
            vec![
                "whl", "egg", // Python
                "gem", "bundle", // Ruby
                "nupkg", "snupkg", // NuGet
                "vsix",   // VS Code
                "crx", "xpi", // Browser extensions
                "aab", // Android App Bundle
                "tgz", "gem", // More package formats
            ],
        ),
        // 💬 Subtitles & Captions (NEW!)
        (
            "Subtitles",
            vec![
                "srt", "sub", "sbv", "ass", "ssa", "vtt", "idx", "sup", "smi", "usf", "ttml",
                "dfxp", "stl", "rt", "lrc",
            ],
        ),
        // 🎨 Shaders & Game Dev (NEW!)
        (
            "Shaders",
            vec![
                "glsl", "hlsl", "cg", "shader", "vert", "frag", "geom", "tesc", "tese", "comp",
                "wgsl", "metal", "spv", "spirv",
            ],
        ),
        // 📧 Email & Communications (NEW!)
        (
            "Email",
            vec![
                "eml",
                "msg",
                "oft",
                "ost",
                "pst",
                "mbox",
                "mbx",
                "emlx",
                "vcf",
                "vcard",
                "ics",
                "ical",
                "icalendar",
            ],
        ),
        // 🔊 Playlists & Media Info (NEW!)
        (
            "Playlists",
            vec![
                "m3u", "m3u8", "pls", "wpl", "asx", "xspf", "cue", "mpcpl", "b4s", "vlc",
            ],
        ),
        // 🧪 Scientific & Research (NEW!)
        (
            "Scientific",
            vec![
                "mat", "sav", // MATLAB, SPSS
                "rdata", "rds", // R
                "nex", "nxs", "newick", "phylip", // Phylogenetic
                "pdb", "cif", "mol", "mol2", "sdf", // Chemistry
                "fcs", "fcs3", // Flow cytometry
                "nii", "dcm", "dicom", // Medical imaging
                "fastq", "fasta", "bam", "sam", "vcf", // Genomics
                "ab1", "scf", "gbk", "embl", // Sequencing
            ],
        ),
    ])
}

/// Optimized: Get category using pre-built map (avoids recreating HashMap)
pub fn get_category_with_map(
    extension: Option<&OsStr>,
    category_map: &HashMap<&'static str, Vec<&'static str>>,
) -> &'static str {
    let ext_str = match extension.and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_lowercase(),
        None => return "Unknown",
    };

    for (&category, extensions) in category_map.iter() {
        if extensions.contains(&ext_str.as_str()) {
            return category;
        }
    }

    "Unknown"
}

// Keep old function for backwards compatibility
pub fn get_category(extension: Option<&OsStr>) -> &'static str {
    let category_map = category_extension_map();
    get_category_with_map(extension, &category_map)
}