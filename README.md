<h1 align="center">League config manager</h1>

<div align="center">
League config manager is a tool that helps manage League of Legends settings.
<br /><br />
  <img alt="App screenshot" src="./assets/app.png" width="50%">
</div>

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Finding League of Legends folder](#finding-league-of-legends-folder)
  - [Locking settings](#locking-settings)
  - [Adding current settings as a profile](#adding-current-settings-as-a-profile)
  - [Link sharing](#link-sharing)
  - [Importing settings](#importing-settings)
  - [Exporting settings](#exporting-settings)
  - [Changing profile](#changing-profile)
  - [Auto-swap](#auto-swap)
- [Troubleshooting](#troubleshooting)
- [Contributing / Feedback](#contributing--feedback)

## Features

This tool aims to help new players get good settings easier, as well as help veteran players manage and share their settings.

Current features:

- Ability to add multiple settings profiles
- Importing/exporting settings
- Generating and sharing a link for a settings profile
- Auto-swap settings per champion
- Locking settings making them reset after every game and same for every account that logs in on that computer

Planned features:

- Pro player/streamer settings website
- Better design of the app

## Installation

1. Go to [releases](https://github.com/brkicdusan/league-config-manager/releases)
1. Open the assets section under the latest release
1. Download `.exe` file
1. To run the file you need to click `More info` (under the text) > `Run anyway` (bottom right) (app is safe to use if you have downloaded it from the official github, you can build the app yourself if you don't trust it, this should disappear after enough people use the app and windows defender stops flagging it)

## Usage

**IMPORTANT**: You shouldn't change profiles in-game. Best way to do it would be to go into practice tool set the settings that you like in game and then add those settings as a profile after you laeve the game.

### Finding League of Legends folder

If League of Legends is not installed not installed at the default directory you will have to select it manually.

To find where the game is installed Go to `Riot Client` > `Profile` (top right) > `Settings` > `League of Legends` > `Install path`

### Locking settings

Lock settings checkbox locks settings making the settings reset to the state you locked them in after every game and after switching to any account on the same computer.

Changing profiles is still possible while this setting is on, the same rules will apply after using a profile.

### Adding current settings as a profile

Press the plus button in the top row to add settings profile.

**IMPORTANT:** This only saves the settings from the time you pressed the button, any changes after won't be saved to the profile, you will have to add a new profile to save them.

### Link sharing

Under every profile you have the button to (G)enerate and (C)opy the link for that profile

_NOTE:_ The link expires are after 7 days
**IMPORANT:** If you get an error importing from link or generating a link you can try again after 1 second if the error persists you can report it here on github, the current solution for sharing links is not meant to serve a lot of people as it uses a rate-limited service

### Importing settings

Press the import button in the top row to import settings from a `.zip` file.

### Exporting settings

Press the export button on the profile that you want to export.

### Changing profile

To use settings from a profile press the use button (second button from the left)

### Auto-swap

To use a specific settings profile for a champion, under that profile select the champion you want to auto-swap on
**IMPORTANT:** Don't forget to set `Default` profile so you can swap back

## Troubleshooting

If you are unsure on how to do something check [Usage](#usage).
If you can't resolve some issue by yourself or you think you have found a bug feel free to open an Issue.

## Contributing / Feedback

This is a passion project that I want to put on my resume, so I am currently not accepting code contributions. This might change if the project gets larger, but at this time there are changes I want to make myself and I don't see a point in taking contributions.

On the other hand all feedback is welcome, if there is a feature or change you would like to see feel free to open an issue. If the project grows in scale there will be a better way to give feedback.
