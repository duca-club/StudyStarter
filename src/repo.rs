use studystarter::Unit;
use reqwest;
use indicatif::{ProgressBar, ProgressStyle};
use std::{cmp::min, time::Duration};
use colored::Colorize;
use std::path::PathBuf;



pub async fn get_unit_manifests(units_string: Vec<String>) -> Result<Vec<Unit>, reqwest::Error> {
    let pb = ProgressBar::new(units_string.len() as u64 * 2);
    pb.set_style(ProgressStyle::with_template(
        "{spinner:.blue} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("=>-"));
    pb.enable_steady_tick(Duration::from_millis(30));

    let mut units: Vec<Unit> = Vec::new();

    let mut pb_position: u64 = 0;
    let pb_max = units_string.len();
    pb.set_position(0);
    for unit in units_string {
        pb.set_message(format!("Downloading: {}", unit));
        
        //Download manifest file
        let manifest_resp = reqwest::get(format!("https://github.com/duca-club/StudyStarter/raw/refs/heads/main/unit_manifests/{}/manifest.txt", unit)).await?;
        let manifest: Vec<PathBuf> = manifest_resp.text().await?.split("\n").map(|x| PathBuf::from(x.to_owned())).collect();
        pb.println(format!("{} manifest.txt for {}", "Downloaded".green().bold(), unit.white().bold()));
        pb_position = min(pb_position+1, pb_max as u64);
        pb.set_position(pb_position);
        
        //Download README file
        let readme_resp = reqwest::get(format!("https://github.com/duca-club/StudyStarter/raw/refs/heads/main/unit_manifests/{}/README.md", unit)).await?;
        let readme = readme_resp.text().await?;
        pb.println(format!("{} README.md for {}", "Downloaded".green().bold(), unit.white().bold()));
        pb_position = min(pb_position+1, pb_max as u64);
        pb.set_position(pb_position);
        
        
        // Create Unit object and save it to the vector
        units.push(Unit {
            name: unit,
            manifest: manifest,
            readme: readme
        });
        
    }

    pb.set_message("Done!");
    pb.finish();

    return Ok(units);
}