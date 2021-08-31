# rustatus
rustatus is a mulithreaded modular status monitor for the Linux tiling window manager, DWM. By default, this program can display the computer's volume percentage, network name, battery percentage, mouse battery percentage, charging status for both the laptop and mouse batter, and the date. This can be customized to include or remove certain plugins or change the refresh rate of each.

## External Dependencies
* xsetroot is the only dependency you may not have on a standard Linux install.
To install on Debian/Ubuntu/Distros with apt
```
apt-get install x11-xserver-utils
```
Arch/Manjaro
```
pacman -S xorg-xsetroot 
```
Fedora
```
dnf install xorg-xsetroot 
```
* Awk, amixer (current commands use pulseaudio) iwgetid are also used. However, all these commands are installed on nearly all Linux desktop installations by default. 

## API design
Each function in lib.rs returns a string. You concatenate each string in main.rs in order to form the $VAR which is then passed to
```
xsetroot -name $VAR
```
which sets the status in DWM. 
Error handling is done in the specific function to make the main.rs file cleaner. 

## How to build  (using Cargo: the Rust build tool and package manager)
```
git clone https://github.com/C-Loftus/rustatus
cd rustatus
cargo build --release
# need sudo to cp into root
sudo cp target/release/rustatus /usr/local/bin/
```
Then add rustatus to your init file (xsession, xinitrc, etc.) that launches DWM.
For example
```
echo "& rustatus" >> .xinitrc
```
# Configuration
You must have a config file named ```config.yaml```
in your ```rustatus/src/``` directory for the program to know which status plugins to include. See the example folder for an example config.
If you do not specify an update rate for a specific plugin, it will update every loop. Make sure you are privy to case-sensitivity in the ```config.yaml``` file. 

