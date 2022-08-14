use std::process::Command;
use std::str;
const RELEVANT_SINK_NAMES: [&str; 2] =
    ["PRO X Wireless Gaming Headset Analog Stereo", "SoundCore 2"];

fn main() {
    change_audio_device();
}

fn change_audio_device(){
    let new_sink_id = switch_default_sink();
    let args = vec!["list", "-t sink-input"];
    let list_inputs_output = exec_with_args("ponymix", args);
    let input_ids = get_input_ids_from_string(&list_inputs_output);
    for id in input_ids {
        switch_input_to_sink(&id, &new_sink_id);
    }
}

fn exec_with_args(cmd: &str, args: Vec<&str>) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("command failed");
    str::from_utf8(&output.stdout).unwrap().to_owned()
}

fn switch_input_to_sink(input: &char, sink: &char) {
    let input_arg = &format!("-d {input}");
    let sink_arg = &format!("move {sink}");
    let args = vec!["-t sink-input", input_arg, sink_arg];

    exec_with_args("ponymix", args);
}

fn get_input_ids_from_string(inputs: &str) -> Vec<char> {
    inputs
        .split("\n")
        .filter(|line| line.contains("sink-input"))
        .map(|line| line.chars().filter(|c| c.is_digit(10)).nth(0).unwrap())
        .collect::<Vec<char>>()
}

fn get_current_default_sink_id() -> char {
    let args = vec!["defaults"];
    exec_with_args("ponymix", args)
        .split("\n")
        .filter(|line| line.contains("sink"))
        .map(|output| output.chars().filter(|c| c.is_digit(10)).nth(0).unwrap())
        .collect::<Vec<char>>()[0]
}

fn get_sink_ids() -> Vec<char> {
    let args = vec!["-t", "sink", "list"];
    let output = exec_with_args("ponymix", args);
    let split = output.split("%");
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
    let mut id_arg = String::new();
    id_arg.push_str("-d ");
    id_arg.push(*new_sink_id);
    let args = vec!["set-default", &*id_arg];
    exec_with_args("ponymix", args);
    return *new_sink_id;
}
