use studystarter::Unit;
use reqwest;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Error;
use std::cmp::min;



pub async fn get_unit_manifests(units_string: Vec<String>) -> Result<Vec<Unit>, Error> {
    let pb = ProgressBar::new(units_string.len() as u64);
    pb.set_style(ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-"));

    let mut units: Vec<Unit> = Vec::new();

    let mut pb_position: u64 = 0;
    let pb_max = units_string.len();
    pb.set_position(0);
    for unit in units_string {
        pb.set_message(format!("Downloading: {}", unit));
        let manifest_resp = reqwest::get(format!("https://github.com/duca-club/StudyStarter/raw/refs/heads/main/unit_manifests/{}/manifest.txt", unit)).await?;
        let readme_resp = reqwest::get(format!("https://github.com/duca-club/StudyStarter/raw/refs/heads/main/unit_manifests/{}/README.md", unit)).await?;

        let manifest = manifest_resp.text().await?.split("\n").map(|x| x.to_owned()).collect();
        let readme = readme_resp.text().await?;

        units.push(Unit {
            name: unit,
            manifest: manifest,
            readme: readme
        });
        pb_position = min(pb_position+1, pb_max as u64);
        pb.set_position(pb_position);
    }

    pb.set_message("Done!");
    pb.finish();

    return Ok(units);
}