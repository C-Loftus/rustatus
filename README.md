# rustatus
This program displays the computer's sound volume percent, network name, battery percentage, mouse battery percentage, charging status for both the laptop and mouse batter, and the date. This is then all displayed in the DWM status bar which is accessed through xsetroot.  

## External Dependencies
xsetroot is the only dependency you may not have on a standard Linux install.
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
Awk, amixer (current commands use pulseaudio) and iwgetid. However, all these commands are installed on nearly all Linux desktop installations by default. 

## API design
Each function in lib.rs returns a string. You concatenate each string in main.rs in order to form the $VAR which is then passed to
```
xsetroot -name $VAR
```
which sets the status in DWM. 
Error handling is done in the specific function to make the main.rs file cleaner. 

## How to use
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

## TODO
This program is usable to my liking but I want to make it better for other users as well. Currently I am working on
* displaying ip addr, location, and weather (all optional of course, just like all other modules)


I plan to potentially add 
* a configuation file system to allow for easier modularity.
* support for different battery configs.
* volume percent support for systems without pulseaudio (just using axmixer master instead of amixer pulse)

