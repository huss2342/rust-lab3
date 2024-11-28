# rust-lab3

## Group Members
- Nick Cochran c.nick@wustl.edu
- Hussein Aljorani a.hussein@wustl.edu
- Becky Shofner r.a.shofner@wustl.edu

## Overview

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


## Usage