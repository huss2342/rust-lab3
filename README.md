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


## Testing



// TODO: fix usage
## Usage 
    1. Unzip the project folder.
    2. Write a script file with its config text files in the root of the project directory, or use the one provided.
    3. Run the main script using the following command:
    ```
   cargo run <script_file_name> [whinge]
    ```
   Where:
    - `<script_file_name>` is the name of your script file (required)
    - `[whinge]` is an optional parameter to enable additional error output

  Example:
    ```
    cargo run partial_hamlet_act_ii_script.txt whinge

    ```