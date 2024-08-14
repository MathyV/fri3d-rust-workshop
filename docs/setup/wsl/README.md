# WSL setup instructions

This is a very rough guide, if you get stuck feel free to reach out but first don't hesitate to use your Google skills ;-) If you are attending the workshop you should probably be able to solve most of the issues you encounter on your own. If I get questions I will add them to the bottom of this page, feel free to create a PR if you came across an issue and found a solution!

## Setup WSL

First of all, if you don't have WSL installed yet, you need to do this. The setup instructions are available [at Microsoft]() but here is the TL;DR:

First, run a Command Prompt in Administrator mode by right clicking the shortcut and selecting `Run as Administrator`
![image](https://github.com/user-attachments/assets/6af91c76-9825-42cf-9721-e7b75f03e034)

In the Command Prompt, run
```
wsl --install
```

Reboot your machine if Windows requests you to do so.

## Launch WSL

You can launch WSL straight from your start menu, when you do, a terminal will open where you drop into a BASH shell.

## Setup Rust

Unsurprisingly, the setup instructions to install Rust inside your VM are exactly the same as under native Linux:

[Linux native Setup Instructions](../linux/)

After you've finished these steps, come back here to finalize your WSL setup.

## Troubleshooting
