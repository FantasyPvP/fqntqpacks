# pack deployer

- a simple command line utility to automatically archive and move your resourcepacks into a folder.

## use cases

this may be useful if you regularly create resourcepacks and want to organise your projects in different folders. 

## how to use

- you'll need a projects folder named whatever you want
- compile the binary in release mode with
 > cargo run --release
- place the binary in the projects folder
- any resourcepacks you're working on should be in subdirectories of this folder
- when the binary is run, any subdirectories containing a pack.mcmeta file will have their contents archived and copied to _resourcepacks in your projects dir
- any existing zip files will also be copied into _resourcepacks
- you should make your .minecraft/resourcepacks folder a shortcut/symlink to _resourcepacks in your projects dir
WARNING: before running, ensure that any packs you want to keep are moved to your projects directory, _resourcepacks will be deleted and replaced each time the binary is run
- any folder prefixed with an underscore will be ignored

here is an example layout:

![image](https://github.com/FantasyPvP/fqntqpacks/assets/80643031/e9603295-abe3-4eeb-872b-9cf75254eca8)


## features
- the utility recursively scans directories in the project folder and organises packs into a folder

## planned features

- reading directory locations to sync packs to from a config file (currently the config part of the code doesn't do anything useful)
- pack repository support (i'm thinking something along the lines of a basic package manager where you can install from a URL or repository)
    - downloading packs
    - publishing / hosting your own packs with sharable URL's (so like a basic webserver kinda thing)
- auto setup: when the app runs for the first time it will do the following
    - ask you to input your resourcepacks folder location
    - ask for a projects folder location
    - move all of your resourcepacks to your projects folder
    - create a shortcut in resourcepacks to your _resourcepacks dir in the deployment folder
    - from there the app will uatomatically add new packs to your folder as you download them like a package manager
