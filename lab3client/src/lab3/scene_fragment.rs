use std::io::Write;
use crate::lab3::declarations::{CharName, FAILED_TO_GENERATE_SCRIPT, WHINGE_MODE};
use crate::lab3::player::Player;
use crate::lab3::script_gen::{grab_trimmed_file_lines, CharacterTextFile,
                              CHARACTER_FILE_CONFIG_LINE_INDEX, CHARACTER_NAME_CONFIG_LINE_INDEX,
                              CONFIG_LINE_TOKENS, CHARACTER_CONFIG_LINE};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

type PlayConfig = Vec<(CharName, CharacterTextFile)>;

pub struct SceneFragment {
    // made public if that's ok
    pub title: String,
    players: Vec<Arc<Mutex<Player>>>,
}

impl SceneFragment {

    /// changed parameter to reference to string and cloned string
    pub fn new(title: &String) -> SceneFragment {
        SceneFragment {
            title: title.clone(),
            players: Vec::new(),
        }
    }

    ///
    /// Print a message for every player that needs to enter for the next scene.
    /// Do this after printing out the title of the scene if applicable.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    /// - `next`: A reference to another instance of the struct SceneFragment
    ///
    pub fn enter(&self, previous: &SceneFragment) {
        // check to see if title contains only whitespace. If not, prints out scene title
        if !self.title.trim().is_empty() {
            writeln!(std::io::stdout().lock(), ).expect("Failed to write to stdout"); // print a newline first to make the printout cleaner
            writeln!(std::io::stdout().lock(), "{}", self.title).expect("Failed to write to stdout");
        }

        for player in &self.players {
            // determine if the previous scene contains the player from the next scene
            let mut contains = false;
            for prev_player in &previous.players {
                if let (Ok(player), Ok(prev_player)) = (player.lock(), prev_player.lock()) {
                    if player.name == prev_player.name {
                        contains = true;
                    }
                }
            }

            if !contains {
                if let Ok(player) = player.lock() {
                    writeln!(std::io::stdout().lock(), "[Enter {}.]", player.name).expect("Failed to write to stdout");
                }
            }
        }

    }

    ///
    /// Print a message for every player in this scene stating that they are entering.
    /// Do this after printing out the title of the scene if applicable.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    ///
    pub fn enter_all(&self) {
        // check to see if title contains only whitespace. If not, prints out scene title
        if !self.title.trim().is_empty() {
            writeln!(std::io::stdout().lock(), ).expect("Failed to write to stdout"); // print a newline first to make the printout cleaner
            writeln!(std::io::stdout().lock(), "{}", self.title).expect("Failed to write to stdout");
        }

        for player in &self.players {
            if let Ok(player) = player.lock() {
                writeln!(std::io::stdout().lock(), "[Enter {}.]", player.name).expect("Failed to write to stdout");
            }
        }
    }

    ///
    /// Print a message for every player that isn't in the next scene to exit.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    /// - `next`: A reference to another instance of the struct SceneFragment
    ///
    pub fn exit(&self, previous: &SceneFragment) {
        for player in previous.players.iter().rev() {
            // determine if the next scene contains the player from the previous scene
            let mut contains = false;
            for next_player in &self.players {
                if let (Ok(next_player), Ok(player)) = (next_player.lock(), player.lock()) {
                    if player.name == next_player.name {
                        contains = true;
                    }
                }
            }
            if !contains {
                if let Ok(player) = player.lock() {
                    writeln!(std::io::stdout().lock(), "[Exit {}.]", player.name).expect("Failed to write to stdout");
                }
            }
        }
    }

    ///
    /// Print a message for every player in this scene stating that they are exiting.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to self
    ///
    pub fn exit_all(&self) {
        for player in self.players.iter().rev() {
            if let Ok(player) = player.lock() {
                writeln!(std::io::stdout().lock(), "[Exit {}.]", player.name).expect("Failed to write to stdout");
            }
        }
    }


    /// create the players vector
    fn process_config(&mut self, play_config: PlayConfig) -> Result<(), u8> {
        for config in play_config {
            match config {
                (char_name, part_file_name) => {
                    let mut player = Player::new(&char_name);
                    if let Err(e) = player.prepare(&part_file_name) {
                        eprintln!("ERROR: Failed to generate script for character {}.",
                                  char_name);
                        return Err(e);
                    }
                    self.players.push(Arc::new(Mutex::new(player)));
                }
            }
        }
        self.players.sort_by(SceneFragment::compare_two_players);
        Ok(())
    }

    fn compare_two_players(player: &Arc<Mutex<Player>>, other: &Arc<Mutex<Player>>) ->  std::cmp::Ordering {
        match (player.lock(), other.lock()) {
            (Ok(player), Ok(other)) => {
                match Player::partial_cmp(&player, &other) {
                    Some(ordering) => ordering,
                    None => std::cmp::Ordering::Equal
                }
            },
            _ => std::cmp::Ordering::Equal,
        }
    }

    /// add a config file to the scene
    fn add_config(&self, config_line: &String, play_config: &mut PlayConfig) {
        let config_line_tokens: Vec<&str> = config_line.split_whitespace().collect();

        if config_line_tokens.len() != CONFIG_LINE_TOKENS {
            if WHINGE_MODE.load(Ordering::SeqCst) {
                writeln!(std::io::stderr().lock(), "Provided config line has the wrong number of tokens.").expect("Failed to write to stderr");
            }
        }

        if config_line_tokens.len() >= CONFIG_LINE_TOKENS {
            play_config.push((
                config_line_tokens[CHARACTER_NAME_CONFIG_LINE_INDEX].to_string(),
                config_line_tokens[CHARACTER_FILE_CONFIG_LINE_INDEX].to_string()
            ))
        }
    }

    /// read from the config file
    fn read_config(&self, config_file_name: &String, play_config: &mut PlayConfig) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();

        match grab_trimmed_file_lines(config_file_name, &mut lines) {
            Ok(()) =>
                {
                    // return error if not enough lines to generate the script
                    if lines.len() <= CHARACTER_CONFIG_LINE { return Err(FAILED_TO_GENERATE_SCRIPT); }

                    // add the config lines to the play configuration data structure
                    Ok(for line in lines {
                        self.add_config(&line, play_config);
                    })
                }
            Err(..) => Err(FAILED_TO_GENERATE_SCRIPT)
        }
    }

    /// call methods to prepare the scene
    pub fn prepare(&mut self, config_file_name: &String) -> Result<(), u8> {
        let mut play_config: PlayConfig = vec![];

        match self.read_config(config_file_name, &mut play_config) {
            Ok(..) => match self.process_config(play_config) {
                Ok(..) => {
                    //  after all Player structs have been added, sort them by lines
                    self.players.sort_by(SceneFragment::compare_two_players);
                    Ok(())
                },
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }

    /// print out the lines for the scene
    pub fn recite(&mut self) {
        let mut cur_line: usize = 0;
        let mut line_exists = true;
        let mut lines_spoken: usize;
        let mut last_speaker = String::new();

        while line_exists {
            line_exists = false;
            lines_spoken = 0;

            for player in &mut self.players {
                if let Ok(mut player) = player.lock() {
                    if let Some(line_num) = player.next_line() {
                        line_exists = true;

                        if cur_line >= line_num {
                            player.speak(&mut last_speaker);
                            lines_spoken += 1;
                        }
                    }
                }
            }
            if WHINGE_MODE.load(Ordering::SeqCst) {
                if lines_spoken == 1 {

                } else {
                    if line_exists {
                        if lines_spoken == 0 {
                            writeln!(std::io::stderr().lock(), "ERROR: Missing line {}", cur_line).expect("Failed to write to stderr");
                        } else {
                            writeln!(std::io::stderr().lock(), "ERROR: Duplicate line on line {}", cur_line).expect("Failed to write to stderr");
                        }
                    }
                }
            }
            cur_line += 1;
        }

    }
}

