# TODO

1. [Jump to Current](#current)
2. [Jump to Future](#future)
3. [Jump to Completed](#completed)
  

## Current
* Add test(s) for existing code
* Feature: options_on_startup
    * ~~write failng test for UI~~
    * develop UI
      * Plan state management between Timer (Widget) and App
        * State only persists in main App so use stateful widget for Timer
    * ensure test passes
    * push
  * Develop options_on_startup()
    * handle startup options
      * handle 'v' (visualizing past sessions)
* Update docs post ratatui integration and refactoring etc...

## Future
* Functionality to visualize and format past study sessions from within the program. \(this is a part of the options_on_startup feature\)
* Official Builds for UNIX and Windows
* Functionality to allow for adding past study sessions, or sessions tracked elsewhere.
* Option to set custom study session length at runtime, and then implement sound cues for session end.
