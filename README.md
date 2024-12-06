# rust-lab3

## Group Members
- Nick Cochran c.nick@wustl.edu
- Hussein Aljorani a.hussein@wustl.edu
- Becky Shofner r.a.shofner@wustl.edu

## Overview
This lab builds on the last two by adding thread safety, multi-threading, and
networking capabilities. We refractored our code from the previous lab and
updated it to use the Arc<Mutex> for safe shared data access, replacing the println!
statements with thread-safe writeln!, and modifying methods to handle locked
references. Multi-threading was implemented to process configuration and file
operations in parallel, with thread managed through join to propogate errors.
A multi-threaded server was developed to handle file requests over a network, 
processing tokens to serve files or shut down based on client input. Local and 
remotevfile I/O are unified through a utility function. 
Each team member then focused on a specific section: Thread-safe output and data sharing, 
Multi-threaded file operations, Multi-threaded server, and From local to networked file 
IO ensuring the code compiled and ran before handing the project off to the next 
partner. Finally, we collaborated on debugging and testing to ensure smooth functionality.

### Thread-Safe Output and Data Sharing
I replaced all `println!` and `eprint!` macros with a `writeln!` macro using
`std::io::stdout()` and `std::io::stderr()` as the output streams 
accordingly. I also added error handling for write operations using 
`expect()`. I then handled thread safety as shown below:

- I modified the `Play` struct by changing the `fragments` vector to 
  store `Arc<Mutex<SceneFragment>>` instead of `SceneFragment`. 

- I modified the `SceneFragment` struct:
  - I changed the `players` vector to store `Arc<Mutex<Player>>` instead of 
    just `Player`
  - Added thread-safe player comparison function:
    ```rust
    fn compare_two_players(player: &Arc<Mutex<Player>>, other: &Arc<Mutex<Player>>) 
        -> std::cmp::Ordering { ... }
    ```

- For both of them I had to:
  - I updated `process_config` to wrap new fragments in `Arc` and `Mutex`
  - I modified `prepare` and `recite` methods to use `lock()` for safe access

### Multi-Threaded File Operations
I modified the Play and SceneFragment structs so that instead of directly 
calling the prepare methods of SceneFragment or Player, each preparation
is done inside a separate thread that was spawned, and each thread's handle
is stored. When joining all the threads, handle panics by propagating upward,
ensuring the current thread also panics. I also changed the prepare funcitons
to use panic! for errors, so they result in thread failures. After I modified
the code, I ran it to ensure correct behavior and that file errors are handled
correctly as before.

Issues I ran into when creating threads were variable ownership
being moved and various functions expecting different types. 
For example, one piece of code expected a String, but the variable was of
type &String. I fixed this by converting the variable to a 
String before assigning it by creating a clone(). Another string
type issue was fixed by adding to_string() to the variable to 
change the type from a &str to a String.

### Multi-Threaded Server

### From Local to Networked File IO
I modified the file I/O to support both local and networked files access. 
This is done by adding the `get_buffered_reader` function which handles both 
network paths and local paths. It does this through using regex to pattern 
match network files. For network files it establishes the TCP connection and 
sends the filename to the server then creates a buffered reader to read from 
the TCP stream. For local files it just simply opens the local file and 
creates the buffered reader. I also modified the `grab_trimed_file_lines` 
function to add support for network files while preserving the same line 
reading behaviour.

## Testing


## Usage

### Setup
1. Unzip the project folder.
2. Write a script file with its config text files in the root of the project directory, or use the one provided.
3. Run either the server, test client, or main client as described below.

### 1- Server
```bash
cargo run <network_address>
# Example
cargo run 127.0.0.1:80
```

### 2- Test Client
```bash
cargo run <network_address> <token>
# Example
cargo run 127.0.0.1:80 filename.txt
```

### 3- Main Client
```bash
cargo run <script_file> [whinge]

# Example: local file
cargo run filename.txt whinge

# Example: network file
cargo run net:127.0.0.1:80:filename.txt
```

**Parameters:**
- `<script_file>` can be:
    - A local file (`partial_hamlet_act_ii_script.txt`)
    - A network file (`net:127.0.0.1:80:partial_hamlet_act_ii_script.txt`)
- `[whinge]` is an optional parameter to enable additional error output

### Important Notes
- Server must be running before connecting with any client
- Port number must be:
    - Available (not in use)
    - Between 0 and 65535
- Both client and server must:
    - Use the same network address and port
    - Have network connectivity
- All files must exist and be accessible