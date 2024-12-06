/*
This client file defines the Play struct, which manages the organization and execution of scene
fragments within a play. It includes methods for preparing scripts, processing configurations,
and reciting scenes, leveraging multithreading and shared state to handle complex interactions
between fragments.
*/

use std::io::Write;
use crate::lab3::declarations::*;
use crate::lab3::script_gen::{grab_trimmed_file_lines};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use crate::lab3::scene_fragment::SceneFragment;
use std::thread;

type ScriptConfig = Vec<(bool, String)>;
type Fragments = Vec<Arc<Mutex<SceneFragment>>>;

pub struct Play {
    fragments: Fragments,
}

impl Play {
    pub fn new() -> Play {
        Play {
            fragments: Vec::new()
        }
    }

    /// modified function for ScriptConfig bool parameters and SceneFragment types
    fn process_config(&mut self, script_config: &ScriptConfig) -> Result<(), u8> {
        let mut title = String::new();
        let mut handles = Vec::new();

        for config in script_config.iter() {

            match config {
                // if true, update scene title
                (true, new_title) => {
                    title = new_title.clone();
                }
                // if false, create a scene fragment and prepare in a thread
                (false, fragment_file_name) => {
                    // create a clone title for thread
                    let title_clone = title.clone();
                    let fragment_file_name_clone = fragment_file_name.clone();
                    // spawn a thread to prepare SceneFragment
                    let handle = thread::spawn(move || {
                        let mut fragment = SceneFragment::new(&title_clone);

                        if let Err(e) = fragment.prepare(&fragment_file_name_clone) {
                            return Err(e);
                        } else {
                            Ok(fragment)
                        }
                    });
                    handles.push(handle);
                    title = String::new();
                }
            }
        }
        // join threads
        for handle in handles {
            match handle.join() {
                Ok(fragment) => {
                    // if Ok push prepared fragment
                    self.fragments.push(Arc::new(Mutex::new(fragment.unwrap())));
                }
                Err(_) => {
                    writeln!(std::io::stderr().lock(), "ERROR: Failed to generate script from file")
                        .expect("Failed to write to stderr");
                    return Err(FAILED_TO_GENERATE_SCRIPT);
                }
            }
        }
        Ok(())
        }




// modified function for ScriptConfig to read in tokens and distinguish between scenes or another config file
    fn add_config(&self, config_line: &String, script_config: &mut ScriptConfig) {
        let config_line_tokens: Vec<&str> = config_line.split_whitespace().collect();
        // ignore blank lines
        if config_line_tokens.is_empty() {
            return;
        }
        // check if first token in the line is [scene]
        if config_line_tokens[SCENE_TOKEN_INDEX] == "[scene]" {

            // if no more tokens, skip and whinge
            if config_line_tokens.len() == SCENE_TITLE_INDEX {
                if WHINGE_MODE.load(Ordering::SeqCst) {
                    writeln!(std::io::stderr().lock(), "Missing scene title.").expect("Failed to write to stderr")
                }
            } else {
                let scene_title = config_line_tokens[SCENE_TITLE_INDEX..].join(" ");
                script_config.push((true, scene_title));
            }
        } else { // if the line is a config file
            let config_file_name = config_line_tokens[CONFIG_FILE_INDEX].to_string();
            script_config.push((false, config_file_name));

            if config_line_tokens.len() > SCRIPT_CONFIG_LINE_TOKENS && WHINGE_MODE.load(Ordering::SeqCst) {
                writeln!(std::io::stderr().lock(), "Provided script has a config line with the wrong number of tokens.").expect("Failed to write to stderr");
            }
        }
    }

    // modified function for ScriptConfig to open script file an read lines
    fn read_config(&self, script_file_name: &String, script_config: &mut ScriptConfig) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();

        match grab_trimmed_file_lines(script_file_name, &mut lines) {
            Ok(()) => {
                if lines.is_empty() {
                    writeln!(std::io::stderr().lock(), "ERROR: Script file '{}' cannot be read", script_file_name).expect("Failed to write to stderr");
                    return Err(FAILED_TO_GENERATE_SCRIPT);
                }

                for line in lines {
                    self.add_config(&line, script_config);
                }
                Ok(())
            }
            Err(e) => {
                writeln!(std::io::stderr().lock(), "ERROR: Failed to open or read script file '{}', error: {}", script_file_name, e).expect("Failed to write to stderr");
                Err(FAILED_TO_GENERATE_SCRIPT)
            }
        }
    }

    // modified function for ScriptConfig to call read_config and check for fragment title
    pub fn prepare(&mut self, script_file_name: &str) -> Result<(), u8> {
        let mut script_config: ScriptConfig = vec![];

        match self.read_config(&script_file_name.to_string(), &mut script_config) {
            Ok(()) => {
                match self.process_config(&script_config) {
                    Ok(()) => {
                        // check for fragments and title
                        if !self.fragments.is_empty() {
                            match self.fragments[FIRST_SCENE_FRAGMENT].lock() {
                                Ok(fragment) => {
                                    if !fragment.title.trim().is_empty() {
                                        Ok(())
                                    } else {
                                        writeln!(std::io::stderr().lock(), "ERROR: First scene fragment has no title").expect("Failed to write to stderr");
                                        Err(FAILED_TO_GENERATE_SCRIPT)
                                    }
                                }
                                Err(..) => {
                                    writeln!(std::io::stderr().lock(), "ERROR: Failed to lock first scene fragment").expect("Failed to write to stderr");
                                    Err(FAILED_TO_GENERATE_SCRIPT)
                                }
                            }
                        } else {
                            writeln!(std::io::stderr().lock(), "ERROR: First scene fragment has no title").expect("Failed to write to stderr");
                            Err(FAILED_TO_GENERATE_SCRIPT)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }


    pub fn recite(&mut self) {
        if self.fragments.is_empty() {
            writeln!(std::io::stderr().lock(), "ERROR: No scene fragments").expect("Failed to write to stderr");
            return;
        }

        // instantiate an iterator
        let mut iter = self.fragments.iter().peekable();
        let mut previous_fragment = None;

        // handle first fragment separately to avoid mutable borrows
        if let Some(fragment) = iter.next() {
            if let Ok(mut fragment_guard) = fragment.lock() {
                fragment_guard.enter_all();
                fragment_guard.recite();
                previous_fragment = Some(fragment);
            }
        }

        // handle last fragment
        while let Some(fragment) = iter.next() {

            if let Some(previous) = &previous_fragment {
                if let (Ok(previous_guard), Ok(fragment_guard)) = (previous.lock(), fragment.lock()) {
                    fragment_guard.exit(&previous_guard);
                    fragment_guard.enter(&previous_guard);
                }
            }

            if let Ok(mut fragment_guard) = fragment.lock() {
                fragment_guard.recite();

                if iter.peek().is_none() {
                    fragment_guard.exit_all();
                }
            }

            previous_fragment = Some(fragment);
        }
    }
}
