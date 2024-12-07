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
Guildenstern Guildenstern_hamlet_ii_2a.txt
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

## 18. Whinge mode
```
PS C:\Users\Navi\Documents\ASchool\rust-lab3\lab3client\src> cargo run partial_hamlet_act_ii_script.txt whinge
   Compiling lab3server v0.1.0 (C:\Users\Navi\Documents\ASchool\rust-lab3\lab3client)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s                                                                                                                                                                                                                                           
     Running `C:\Users\Navi\Documents\ASchool\rust-lab3\lab3client\target\debug\lab3server.exe partial_hamlet_act_ii_script.txt whinge`

Hamlet Prince of Denmark ACT II Scene I A room in Polonius house by William Shakespeare
[Enter Polonius.]
[Enter Reynaldo.]
ERROR: Missing line 0

Polonius:
Give him this money and these notes, Reynaldo.

Reynaldo:
I will, my lord.

Polonius:
You shall do marvellous wisely, good Reynaldo,
Before You visit him, to make inquiry
Of his behaviour.

Reynaldo:
My lord, I did intend it.

Polonius:
Marry, well said; very well said. Look you, sir,
Enquire me first what Danskers are in Paris;
And how, and who, what means, and where they keep,
What company, at what expense; and finding,
By this encompassment and drift of question,
That they do know my son, come you more nearer
Than your particular demands will touch it:
Take you, as 'twere, some distant knowledge of him;
As thus, 'I know his father and his friends,
And in part hi;m;--do you mark this, Reynaldo?

Reynaldo:
Ay, very well, my lord.

Polonius:
'And in part him;--but,' you may say, 'not well:
But if't be he I mean, he's very wild;
Addicted so and so;' and there put on him
What forgeries you please; marry, none so rank
As may dishonour him; take heed of that;
But, sir, such wanton, wild, and usual slips
As are companions noted and most known
To youth and liberty.

Reynaldo:
As gaming, my lord.

Polonius:
Ay, or drinking, fencing, swearing, quarrelling,
Drabbing:--you may go so far.

Reynaldo:
My lord, that would dishonour him.

Polonius:
Faith, no; as you may season it in the charge.
You must not put another scandal on him,
That he is open to incontinency;
That's not my meaning: but breathe his faults so quaintly
That they may seem the taints of liberty;
The flash and outbreak of a fiery mind;
A savageness in unreclaimed blood,
Of general assault.

Reynaldo:
But, my good lord,--

Polonius:
Wherefore should you do this?

Reynaldo:
Ay, my lord,
I would know that.

Polonius:
Marry, sir, here's my drift;
And I believe it is a fetch of warrant:
You laying these slight sullies on my son
As 'twere a thing a little soil'd i' the working,
Mark you,
Your party in converse, him you would sound,
Having ever seen in the prenominate crimes
The youth you breathe of guilty, be assur'd
He closes with you in this consequence;
'Good sir,' or so; or 'friend,' or 'gentleman'--
According to the phrase or the addition
Of man and country.

Reynaldo:
Very good, my lord.

Polonius:
And then, sir, does he this,--he does--What was I about to say?--
By the mass, I was about to say something:--Where did I leave?

Reynaldo:
At 'closes in the consequence,' at 'friend or so,' and
gentleman.'

Polonius:
At--closes in the consequence'--ay, marry!
He closes with you thus:--'I know the gentleman;
I saw him yesterday, or t'other day,
Or then, or then; with such, or such; and, as you say,
There was he gaming; there o'ertook in's rouse;
There falling out at tennis': or perchance,
'I saw him enter such a house of sale,'--
Videlicet, a brothel,--or so forth.--
See you now;
Your bait of falsehood takes this carp of truth:
And thus do we of wisdom and of reach,
With windlaces, and with assays of bias,
By indirections find directions out:
So, by my former lecture and advice,
Shall you my son. You have me, have you not?

Reynaldo:
My lord, I have.

Polonius:
God b' wi' you, fare you well.

Reynaldo:
Good my lord!

Polonius:
Observe his inclination in yourself.

Reynaldo:
I shall, my lord.

Polonius:
And let him ply his music.

Reynaldo:
Well, my lord.

Polonius:
Farewell!
[Exit Reynaldo.]
[Enter Ophelia.]
ERROR: Missing line 0

Polonius:
How now, Ophelia! what's the matter?

Ophelia:
Alas, my lord, I have been so affrighted!

Polonius:
With what, i' the name of God?

Ophelia:
My lord, as I was sewing in my chamber,
Lord Hamlet,--with his doublet all unbrac'd;
No hat upon his head; his stockings foul'd,
Ungart'red, and down-gyved to his ankle;
Pale as his shirt; his knees knocking each other;
And with a look so piteous in purport
As if he had been loosed out of hell
To speak of horrors,--he comes before me.

Polonius:
Mad for thy love?

Ophelia:
My lord, I do not know;
But truly I do fear it.

Polonius:
What said he?

Ophelia:
He took me by the wrist, and held me hard;
Then goes he to the length of all his arm;
And with his other hand thus o'er his brow,
He falls to such perusal of my face
As he would draw it. Long stay'd he so;
At last,--a little shaking of mine arm,
And thrice his head thus waving up and down,--
He rais'd a sigh so piteous and profound
As it did seem to shatter all his bulk
And end his being: that done, he lets me go:
And, with his head over his shoulder turn'd
He seem'd to find his way without his eyes;
For out o' doors he went without their help,
And to the last bended their light on me.

Polonius:
Come, go with me: I will go seek the king.
This is the very ecstasy of love;
Whose violent property fordoes itself,
And leads the will to desperate undertakings,
As oft as any passion under heaven
That does afflict our natures. I am sorry,--
What, have you given him any hard words of late?

Ophelia:
No, my good lord; but, as you did command,
I did repel his letters and denied
His access to me.

Polonius:
That hath made him mad.
I am sorry that with better heed and judgment
I had not quoted him: I fear'd he did but trifle,
And meant to wreck thee; but beshrew my jealousy!
It seems it as proper to our age
To cast beyond ourselves in our opinions
As it is common for the younger sort
To lack discretion. Come, go we to the king:
This must be known; which, being kept close, might move
More grief to hide than hate to utter love.
[Exit Ophelia.]
[Exit Polonius.]

Hamlet Prince of Denmark ACT II Scene II A room in the Castle by William Shakespeare
[Enter King.]
[Enter Queen.]
[Enter Rosencrantz.]
[Enter Guildenstern.]
ERROR: Missing line 0

King:
Welcome, dear Rosencrantz and Guildenstern!
Moreover that we much did long to see you,
The need we have to use you did provoke
Our hasty sending. Something have you heard
Of Hamlet's transformation; so I call it,
Since nor the exterior nor the inward man
Resembles that it was. What it should be,
More than his father's death, that thus hath put him
So much from the understanding of himself,
I cannot dream of: I entreat you both
That, being of so young days brought up with him,
And since so neighbour'd to his youth and humour,
That you vouchsafe your rest here in our court
Some little time: so by your companies
To draw him on to pleasures, and to gather,
So much as from occasion you may glean,
Whether aught, to us unknown, afflicts him thus,
That, open'd, lies within our remedy.

Queen:
Good gentlemen, he hath much talk'd of you,
And sure I am two men there are not living
To whom he more adheres. If it will please you
To show us so much gentry and good-will
As to expend your time with us awhile,
For the supply and profit of our hope,
Your visitation shall receive such thanks
As fits a king's remembrance.

Rosencrantz:
Both your majesties
Might, by the sovereign power you have of us,
Put your dread pleasures more into command
Than to entreaty.

Guildenstern:
We both obey,
And here give up ourselves, in the full bent,
To lay our service freely at your feet,
To be commanded.

King:
Thanks, Rosencrantz and gentle Guildenstern.

Queen:
Thanks, Guildenstern and gentle Rosencrantz:
And I beseech you instantly to visit
My too-much-changed son.--Go, some of you,
And bring these gentlemen where Hamlet is.

Guildenstern:
Heavens make our presence and our practices
Pleasant and helpful to him!

Queen:
Ay, amen!
[Exit Guildenstern.]
[Exit Rosencrantz.]
[Exit Queen.]
[Exit King.]
```