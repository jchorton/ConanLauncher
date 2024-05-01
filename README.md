
# What is this?

A Conan Exiles launcher with a fix. Licensed under the GPLv3.

![image](https://github.com/Davenport-Physics/ConanLauncher/assets/1410579/e0167d91-811e-43d9-9bf0-db5c252eaf19)


# What's the fix?

Have you ever been annoyed by the startup video played when you launch the game? Have you gone out of your way to disable it, only for a pesky update to revert your configuration changes? 

This launcher disables those startup videos every time you launch the game. You never have to be told you've been left to die, again!

# Does it do anything else?

The launcher will also attempt to launch steam if steam isn't already running.

# Public Version

This software was originally designed for me, the developer! What that means for you, the user, is that there are features in the master branch that are presently disabled in the public release version.

Features such as

- Support for writing emotes (for those of you that want spellcheck)
- Support for the Tot!Chat webhook (for automatically saving other character's emotes)

How do I get the master branch version? Ask or consider building it yourself!

# Smart Screen

Help! It says this software comes from an untrusted publisher! Is it safe to run?

Well, this requires a nuanced answer. Microsoft now requires software to be digitally signed with a code certificate. Unfortunately, acquiring a code certificate is cost-prohibitive at a staggering $300 a year. This software is available free of charge and was made to scratch a particular itch. Furthermore, the code is available for review and you can build the software yourself.

So is it safe to run? You should decide for yourself!

Here is a complimentary [virustotal.com report](https://www.virustotal.com/gui/file/d1a7307bfd57ff4241c455fb18f5bbc9b99f459ef662c5031506513811067696/detection)

# Build requirements

- Install [Node.JS](https://nodejs.org/en/download/current)
- Install [Rust](https://www.rust-lang.org/learn/get-started)

# Build

In the directory of the downloaded code, run the following command in cmd

```
npm run tauri build
```

this will place an executable under src-tauri/target/release




