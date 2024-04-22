use json::object;
use std::process::Command;

const COMMAND_ERROR: &str = "Could not run playerctl!
Please make sure it is intalled in the first place.
Please refer to your package manager for installation instructions.";

fn main() {
    let title_command = Command::new("playerctl")
        .args(["metadata", "title"])
        .output()
        .expect(COMMAND_ERROR);
    let temp_title = String::from_utf8(title_command.stdout).unwrap();
    let mut title = temp_title.trim().to_string();
    let artist_command = Command::new("playerctl")
        .args(["metadata", "artist"])
        .output()
        .expect(COMMAND_ERROR);
    let artist = String::from_utf8(artist_command.stdout).unwrap();
    if !artist.trim().is_empty() {
        title.push_str(format!(" - {}", artist.trim()).as_str());
    }
    let output = object! {"text": title, tooltip:"", class:"media"};
    println!("{}", output.dump());
}
