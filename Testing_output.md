# Testing File

## 1. No files provided to client program
```
PS C:\rust-lab3\lab3client\src> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe`
usage: C:\rust-lab3\lab3client\target\debug\lab3server.exe <script_file> [whinge]
(script_file can be local or net:host:port:filename)
ERROR: Bad command line arguments provided.
Error: 1
error: process didn't exit successfully: `C:\rust-lab3\lab3client\target\debug\lab3server.exe` (exit code: 1)
```

## 2. Provided more than 1 file
```
PS C:\rust-lab3\lab3client\src> cargo run partial_hamlet_act_ii_script.txt partial_macbeth_act_i_script.txt  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe partial_hamlet_act_ii_script.txt partial_macbeth_act_i_script.txt`
usage: C:\rust-lab3\lab3client\target\debug\lab3server.exe <script_file> [whinge]
(script_file can be local or net:host:port:filename)
ERROR: Bad command line arguments provided.
Error: 1
error: process didn't exit successfully: `C:\rust-lab3\lab3client\target\debug\lab3server.exe partial_hamlet_act_ii_script.txt partial_macbeth_act_i_script.txt` (exit code: 1)
```

## 3. Invalid script file path provided
```
PS C:\rust-lab3\lab3client\src> cargo run nonExistingFile.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe nonExistingFile.txt`
ERROR: Failed to open file 'nonExistingFile.txt': 4
ERROR: Failed to open or read script file 'nonExistingFile.txt', error: 2
Error: 2
error: process didn't exit successfully: `C:\rust-lab3\lab3client\target\debug\lab3server.exe nonExistingFile.txt` (exit code: 2)
PS C:\rust-lab3\lab3client\src> 
```

## 4. Script partial file with badly formed config file paths (none exist)
```
PS C:\rust-lab3\lab3client\src> cargo run TEST_partial_hamlet_act_ii_script.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe TEST_partial_hamlet_act_ii_script.txt`
ERROR: Failed to open file 'DOESNOTEXIST_hamlet_ii_2a_config.txt': 4
ERROR: Failed to open file 'thread 'DOESNOTEXIST_hamlet_ii_1a_config.txt<unnamed>': ' panicked at 4src\lab3\scene_fragment.rs
:thread '243<unnamed>:' panicked at 23src\lab3\scene_fragment.rs:
:Failed to read configuration file: DOESNOTEXIST_hamlet_ii_2a_config.txt243
:note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
23:
Failed to read configuration file: DOESNOTEXIST_hamlet_ii_1a_config.txt
ERROR: Failed to generate script from file
Error: 2
error: process didn't exit successfully: `C:\rust-lab3\lab3client\target\debug\lab3server.exe TEST_partial_hamlet_act_ii_script.txt` (exit code: 2)
```

## 4. Script partial file with badly formed config file paths (some exist)
```
PS C:\rust-lab3\lab3client\src> cargo run TEST_partial_hamlet_act_ii_script.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe TEST_partial_hamlet_act_ii_script.txt`
ERROR: Failed to open file 'DOESNOTEXIST_hamlet_ii_1b_config.txt': 4
thread '<unnamed>' panicked at src\lab3\scene_fragment.rs:243:23:
Failed to read configuration file: DOESNOTEXIST_hamlet_ii_1b_config.txt
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
ERROR: Failed to generate script from file
Error: 2
error: process didn't exit successfully: `C:\rust-lab3\lab3client\target\debug\lab3server.exe TEST_partial_hamlet_act_ii_script.txt` (exit code: 2)
```

## 5. Config file with badly formed part file paths
```
PS C:\rust-lab3\lab3client\src> cargo run TEST_partial_hamlet_act_ii_script.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe TEST_partial_hamlet_act_ii_script.txt`
ERROR: Failed to open file 'DOESNOTEXIST_Rosencrantz_hamlet_ii_2a.txt': 4
thread '<unnamed>' panicked at src\lab3\player.rs:110:13:
Failed to read the part file: DOESNOTEXIST_Rosencrantz_hamlet_ii_2a.txt
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
ERROR: Failed to open file 'DOESNOTEXIST_Guildenstern_hamlet_ii_2a.txt': 4
thread '<unnamed>' panicked at src\lab3\player.rs:110:13:
Failed to read the part file: DOESNOTEXIST_Guildenstern_hamlet_ii_2a.txt
ERROR: Failed to generate script for player
thread '<unnamed>' panicked at src\lab3\scene_fragment.rs:241:28:
Failed to process configuration file: TEST_hamlet_ii_2a_config.txt
ERROR: Failed to generate script from file
Error: 2
error: process didn't exit successfully: `C:\rust-lab3\lab3client\target\debug\lab3server.exe TEST_partial_hamlet_act_ii_script.txt` (exit code: 2)      
PS C:\rust-lab3\lab3client\src>
```

## 6. Successfully reading a partial script file
```
PS C:\rust-lab3\lab3client\src> cargo run partial_hamlet_act_ii_script.txt     
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe partial_hamlet_act_ii_script.txt`

Hamlet Prince of Denmark ACT II Scene I A room in Polonius house by William Shakespeare
[Enter Polonius.]
[Enter Reynaldo.]

... The rest of the play ...
```

## 7. Starting server with no arguments
```
PS C:\rust-lab3\lab3server> cargo run
   Compiling lab3server v0.1.0 (C:\rust-lab3\lab3server)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s                                                                                
     Running `target\debug\lab3server.exe`
usage: target\debug\lab3server.exe <network_address>
Error: 1
error: process didn't exit successfully: `target\debug\lab3server.exe` (exit code: 1)
```
    
## 8. Server basic startup test
```
PS C:\rust-lab3\lab3server\src> cargo run 127.0.0.1:80
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\lab3server.exe 127.0.0.1:80`
Server listening on: 127.0.0.1:80
```

## 9. Server receiving an invalid file request
**Client Output**
```
PS C:\rust-lab3\lab3client\src> cargo run net:127.0.0.1:80:nonExistingFile                                   
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe net:127.0.0.1:80:nonExistingFile`
ERROR: Script file 'net:127.0.0.1:80:nonExistingFile' cannot be read
Error: 2
error: process didn't exit successfully: `C:\rust-lab3\lab3client\target\debug\lab3server.exe net:127.0.0.1:80:nonExistingFile` (exit code: 2)

```
**Server Output**
```
PS C:\rust-lab3\lab3server\src> cargo run 127.0.0.1:80
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\lab3server.exe 127.0.0.1:80`
Server listening on: 127.0.0.1:80
ERROR: Failed to open file 'nonExistingFile': The system cannot find the file specified. (os error 2)
```

## 10. Server receiving valid file request
**Client Output**
```
PS C:\rust-lab3\lab3client\src> cargo run net:127.0.0.1:80:partial_hamlet_act_ii_script.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe net:127.0.0.1:80:partial_hamlet_act_ii_script.txt`

Hamlet Prince of Denmark ACT II Scene I A room in Polonius house by William Shakespeare
[Enter Polonius.]
[Enter Reynaldo.]

... The rest of the play ...
```
**Server Output**
```
PS C:\rust-lab3\lab3server\src> cargo run 127.0.0.1:80                                     
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `C:\rust-lab3\lab3server\target\debug\lab3server.exe 127.0.0.1:80`
Server listening on: 127.0.0.1:80
sending file: partial_hamlet_act_ii_script.txt
```

## 11. Running client_test with no arguments
```
PS C:\rust-lab3\lab3client_test> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target\debug\lab3client_test.exe`
usage: target\debug\lab3client_test.exe <network_address> <token>
Error: 1
error: process didn't exit successfully: `target\debug\lab3client_test.exe` (exit code: 1)
```

## 12. Running client_test requesting a file from server
**Client Output**
```
PS C:\rust-lab3\lab3client_test> cargo run 127.0.0.1:80 TEST_file.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\lab3client_test.exe 127.0.0.1:80 TEST_file.txt`
WRITING: 
Hello, this is the content of this test file.
```
**Server Output**
```
PS C:\rust-lab3\lab3server\src> cargo run 127.0.0.1:80
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `C:\rust-lab3\lab3server\target\debug\lab3server.exe 127.0.0.1:80`
Server listening on: 127.0.0.1:80
sending file: TEST_file.txt
```

## 12. Client_test sending "quit" command
**Client Output**
```
PS C:\rust-lab3\lab3client_test> cargo run 127.0.0.1:80 quit
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\lab3client_test.exe 127.0.0.1:80 quit`
```
**Server Output**
``` 
PS C:\rust-lab3\lab3server\src> cargo run 127.0.0.1:80
   Compiling lab3server v0.1.0 (C:\rust-lab3\lab3server)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `C:\rust-lab3\lab3server\target\debug\lab3server.exe 127.0.0.1:80`
Server listening on: 127.0.0.1:80
Quitting...
```


## 17. Mixed local and remote file access
**Config Content**
```
Guildenstern DOESNOTEXIST_Guildenstern_hamlet_ii_2a.txt
King net:127.0.0.1:80:King_hamlet_ii_2a.txt
Queen Queen_hamlet_ii_2a.txt
Rosencrantz net:127.0.0.1:80:Rosencrantz_hamlet_ii_2a.txt
```
**Client Output**
```
PS C:\rust-lab3\lab3client\src> cargo run TEST_partial_hamlet_act_ii_script.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:\rust-lab3\lab3client\target\debug\lab3server.exe TEST_partial_hamlet_act_ii_script.txt`

Hamlet Prince of Denmark ACT II Scene I A room in Polonius house by William Shakespeare
[Enter Polonius.]
[Enter Reynaldo.]

... The rest of the play ...
```
**Server Output**
``` 
PS C:\rust-lab3\lab3server\src> cargo run 127.0.0.1:80
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `C:\rust-lab3\lab3server\target\debug\lab3server.exe 127.0.0.1:80`
Server listening on: 127.0.0.1:80
sending file: Rosencrantz_hamlet_ii_2a.txt
sending file: King_hamlet_ii_2a.txt
```