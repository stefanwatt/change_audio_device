use std::process::Command;
use std::str;
const RELEVANT_SINK_NAMES: [&str; 2] =
    ["PRO X Wireless Gaming Headset Analog Stereo", "SoundCore 2"];

fn main() {
    let new_sink_id = switch_default_sink();
    let list_inputs_output = exec_with_args("ponymix", &["list", "-t sink-input"]);
    let input_ids = get_input_ids_from_string(&list_inputs_output);
    for id in input_ids {
        switch_input_to_sink(id, new_sink_id);
    }
}

fn exec_with_args(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("command failed");
    str::from_utf8(&output.stdout).unwrap().to_owned()
}

fn switch_input_to_sink(input: char, sink: char) {
    exec_with_args("ponymix move {sink}", &["-t sink-input", "-d {input}"]);
}

fn get_input_ids_from_string(inputs: &str) -> Vec<char> {
    inputs
        .split("\n")
        .filter(|line| line.contains("sink-input"))
        .map(|line| line.chars().filter(|c| c.is_digit(10)).nth(0).unwrap())
        .collect::<Vec<char>>()
}

fn get_current_default_sink_id() -> char {
    exec_with_args("ponymix", &["defaults"])
        .split("\n")
        .filter(|line| line.contains("sink"))
        .map(|output| output.chars().filter(|c| c.is_digit(10)).nth(0).unwrap())
        .collect::<Vec<char>>()[0]
}

fn get_sink_ids() -> Vec<char> {
    let output = exec_with_args("ponymix", &["-t", "sink", "list"]);
    let split = output.split("sink");
    let chunks = split
        .filter(|chunk| {
            chunk.contains(RELEVANT_SINK_NAMES[0]) || chunk.contains(RELEVANT_SINK_NAMES[1])
        })
        .collect::<Vec<&str>>();
    chunks
        .iter()
        .map(|chunk| {
            chunk
                .split("\n")
                .filter(|line| line.contains(": "))
                .map(|output| output.chars().filter(|c| c.is_digit(10)).nth(0).unwrap())
                .collect::<Vec<char>>()[0]
        })
        .collect::<Vec<char>>()
}

fn switch_default_sink() -> char {
    let current_default_sink_id = get_current_default_sink_id();
    let sink_ids = get_sink_ids();
    let new_sink_id = sink_ids
        .iter()
        .filter(|&&id| id != current_default_sink_id)
        .collect::<Vec<&char>>()[0];
    exec_with_args("ponymix", &["set-default", "-d {new_sink_id}"]);
    return *new_sink_id;
}