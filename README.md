# pack deployer

- a simple command line utility to automatically archive and move your resourcepacks into a folder.

## use cases

this may be useful if you regularly create resourcepacks and want to organise your projects in different folders. 

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
