use clap::{App, Arg};
use mslnk::ShellLink;
use std::error::Error;
use std::fs;
use steam_shortcuts_util::parse_shortcuts;
use steam_shortcuts_util::Shortcut;
extern crate ini;
use ini::Ini;

const SHORCTUTS_FILE: &str = r"shortcuts.vdf";
const STREAMING_ASSETS_SUBDIR: &str = r"StreammingAssets\";

fn wrap_with_marks(text: &str) -> String {
    let mut res = "\"".to_owned();
    res.push_str(text);
    res.push_str("\"");
    return res;
}

fn create_link(
    input_target: &str,
    steam_config_dir: &str,
    dest_folder: &str,
    sc: Shortcut,
) -> Result<(), Box<dyn Error>> {
    let target = input_target.to_string();
    let wrapped_target = &wrap_with_marks(input_target)[..];
    let arguments = sc.exe.clone().replace(wrapped_target, "");
    let icon_location = sc.icon.to_string();
    let name = sc.app_name.to_string();

    let mut escaped_name = name.replace("\\", "");
    escaped_name = escaped_name.replace("/", "");
    escaped_name = escaped_name.replace(":", "");
    escaped_name = escaped_name.replace("*", "");
    escaped_name = escaped_name.replace("?", "");
    escaped_name = escaped_name.replace("\"", r"");
    escaped_name = escaped_name.replace("<", "");
    escaped_name = escaped_name.replace(">", "");
    escaped_name = escaped_name.replace("|", "");
    let working_dir = sc.start_dir.to_string();

    let mut lnk = "".to_owned();
    lnk.push_str(dest_folder);
    lnk.push_str(&escaped_name);
    lnk.push_str(r".lnk");

    let mut sl = ShellLink::new(target)?;
    sl.set_arguments(Some(arguments));
    sl.set_icon_location(Some(icon_location));
    sl.set_name(Some(name));
    sl.set_working_dir(Some(working_dir));
    sl.create_lnk(lnk)?;

    // Create Box Art
    let mut box_art_dir = "".to_owned();
    box_art_dir.push_str(dest_folder);
    box_art_dir.push_str(STREAMING_ASSETS_SUBDIR);
    box_art_dir.push_str(&escaped_name);
    fs::create_dir_all(&box_art_dir)?;

    let mut input_boxart_path = "".to_owned();
    input_boxart_path.push_str(steam_config_dir);
    input_boxart_path.push_str(r"grid\");
    input_boxart_path.push_str(&sc.app_id.to_string());

    let mut input_boxart_png = input_boxart_path.clone();
    input_boxart_png.push_str("p.png");

    let mut input_boxart_jpg = input_boxart_path.clone();
    input_boxart_jpg.push_str("p.jpg");

    let mut box_art_png = box_art_dir.clone();
    box_art_png.push_str(r"\box-art.png");

    if std::path::Path::new(&input_boxart_png).exists() {
        let box_art_png = box_art_png.trim();
        fs::copy(&input_boxart_png, box_art_png)?;
    } else if std::path::Path::new(&input_boxart_jpg).exists() {
        let img = image::open(input_boxart_jpg).unwrap();
        img.save(box_art_png).unwrap();
    } else {
        let box_art_png = box_art_png.trim();
        fs::copy("assets/box-art.png", box_art_png)?;
    }

    Ok(())
}

fn process_target(
    target: &str,
    steam_config_dir: &str,
    dest_folder: &str,
) -> Result<(), Box<dyn Error>> {
    let mut shortcuts_path = "".to_owned();
    shortcuts_path.push_str(steam_config_dir);
    shortcuts_path.push_str(SHORCTUTS_FILE);

    let content = std::fs::read(shortcuts_path)?;
    let shortcuts = parse_shortcuts(content.as_slice())?;

    let entries: Vec<Shortcut> = shortcuts
        .into_iter()
        .filter(|sc| sc.exe.contains(target))
        .collect();
    if entries.len() == 0 {
        Err("Target does not exist.")?;
    }
    for sc in entries {
        create_link(target, steam_config_dir, dest_folder, sc)?;
    }

    Ok(())
}

fn main() {
    let matches = App::new("nvidia-gamestream-presets-tool: NVIDIA GameStream Presets Tool")
    .version("0.1.0")
    .author("Uklosk <jomai92@gmail.com>")
    .about("Create NVIDIA GameStream Presets from Steam Shortcuts. Useful with SteamRomManager")
    .arg(Arg::with_name("targets")
         .short('t')
         .long("targets")
         .value_name("TARGETS")
         .help(r"Paths of the executables to match divided by commas. Overrides TARGET in conf.ini. e.g. C:\Emus\Yuzu\Yuzu.exe,C:\Emus\Cemu\Cemu.exe")
         .multiple(false)
         .required(false)
         .takes_value(true)
      .min_values(1))
    .arg(Arg::with_name("steam_config_dir")
         .short('s')
         .long("steam-config-dir")
         .value_name("STEAM_CONFIG_DIR")
         .help(r"Path to where the Steam's shortcuts.vdf is located. e.g. C:\\Program Files (x86)\\Steam\\userdata\\37089855\\config\\")
         .multiple(false)
         .required(false)
         .takes_value(true)
      .min_values(1))
    .arg(Arg::with_name("dest_folder")
         .short('d')
         .long("dest-folder")
         .value_name("DEST_FOLDER")
         .help(r"Path to the folder where the GameStream shortcuts and assets are going to be placed. e.g. C:\\Users\\username\\AppData\\Local\\NVIDIA Corporation\\Shield Apps\\")
         .multiple(false)
         .required(false)
         .takes_value(true)
      .min_values(1))
    .arg(Arg::with_name("config")
         .short('c')
         .long("config-file")
         .value_name("CONFIG_FILE")
         .help("Path to the conf.ini. default: conf.ini")
         .multiple(false)
         .required(false)
         .takes_value(true)
         .default_value("conf.ini")
      .min_values(1))
    .get_matches();

    let conf_path = matches.value_of("config").unwrap();

    // Read INI
    let conf = Ini::load_from_file(conf_path).unwrap();
    let section = conf.section(Some("Config")).unwrap();

    // Targets Applications
    let targets: Vec<&str>;
    if matches.is_present("targets") {
        targets = matches.value_of("targets").unwrap().split(",").collect();
    } else {
        targets = section.get("TARGETS").unwrap().split(",").collect();
    }

    // Steam Config Dir
    let steam_config_dir: &str;
    if matches.is_present("steam_config_dir") {
        steam_config_dir = matches.value_of("steam_config_dir").unwrap();
    } else {
        steam_config_dir = section.get("STEAM_CONFIG_DIR").unwrap();
    }

    // Destination Folder
    let dest_folder: &str;
    if matches.is_present("dest_folder") {
        dest_folder = matches.value_of("dest_folder").unwrap();
    } else {
        dest_folder = section.get("DEST_FOLDER").unwrap();
    }
    for t in targets {
        let target = t.trim();

        let r = process_target(target, steam_config_dir.trim(), dest_folder.trim());

        match r {
            Ok(file) => file,
            Err(error) => panic!("Error processing target {:?}, error: {:?}", target, error),
        };
    }
}
