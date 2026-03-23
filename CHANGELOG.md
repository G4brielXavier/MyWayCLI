## myway-cli v0.1.0 Stable:

- Add features 
    - Separate two commands to finish a project.
        - `myway finish` to mark a project as finish.
        - `myway status` to write (custom) status of project (as "stable", "deprecated").

    - Add `myway version`:
        - `--list`: show all versions of a specific project.
        - `--add`: add a new version to a specific project.

    - Add `myway graveyard` to visit projects "killed" (just a list).
        - `--kill`: Kill a project, remove from Way and add to graveyard.
        - `--list`: List all projects in graveyard.

    - Add "myway way --complex" to see more details about project.

- Improvements
    - If project's name has spaces, replace to '-' ("My Project" -> "My-Project") [DONE!]
    - Logs improved
    - 'myway giveup', 'myway way' improved
    - Improvements in filters 
    - "MyWayError" added to improve UX and errors logs


## myway-cli v0.1.6:

  - Changed prefix `myway` to `mw`

  - Improvements
      - README fixes
      - Changed `create` to `add`
      - Change `(F)` and `Status: new` to `(W : new)`


## myway-cli v0.2.0:

- New Feature:
    - Added 3 new parameters to `mw way`:
        - `--status <STATUS>`: filter by specific status
        - `--finish`: show only projects that is `F`
        - `--working`: show only projects that is `W` 

- New Command - `mw ord`: to ordenate projects on WAY
    - `--first`: Put a project as first
    - `--last`: Put a project as last
    - `--swap`: Swap two projects

- New Command - `mw reviv`: to revive a specific project from graveyard


- Improvements
    - Now was defined a limit of characters
        - `project_name`: <=12
        - `status`: <=8
        - `version`: <=8
        - `description`: <=24

    - No spaces to `status` and `version`
    - UX CLI Experience improved


## myway-cli v0.3.0:

- Security: The MyWay's internal file are encrypted by **Tequel**

- Improvements
  - UX Colors improved (`colored`)
  - `mw giveup` visual improved

## myway-cli v0.3.1

- Change Limits:
  - Description: <=35
  - Project's Name: <=20

## myway-cli v0.3.3

- Added Latest Release Verifier (`crates.io` API)
- Some improvements
- Bugs fixes

## myway-cli v0.3.4

- New Parameter to `mw hey`:
  - `--update`: Verify if `myway` is updated