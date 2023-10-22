# System for tracking log in/out records in SQLite files
In its current state it will take in a string consisting of a character (either i or o) to signify logging in or out, and an integer ID value (ex: 12345). It will then store those logs in a file called `test.db`. 

Upon exiting this stage with an exit code `X`, the logs will be processed into a list of sessions with duration values in seconds.
## Info/help
- This program is intended to be used with barcodes, but anything that can enter text into a field will work
- The data is only currently accessible through a program able to view SQLite db files
- The program is only runnable through a command line currently
## Future plans
- Optimise constructing the list of sessions (it currently reconstructs the whole list every time)
- User registration and barcode/id card generation
- Ability to display and analyze session data
- Config
- Gui