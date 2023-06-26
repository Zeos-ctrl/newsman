# Newsman User Guide

![anchorman](./UserGuide/images/60everytime.gif)

Newsman is a command line program that allows you to construct a mailing list and send 
out newsletters to the people on that list. It can work attached to the terminal 
or detached as a daemon and run as a background process!

## Installation
Newsman can be installed from source for anyone familiar with that process,
a database must also be constructed in order to complete the installation.
The database schema can be found in the git repo under newsman.sql and 
MUST BE INSTALLED INTO A DATABASE BASED ON MYSQL, at the moment 
that's the only supported database type, i.e, mariadb. After running the 
program for the first time, newsman should prompt you to make config files in 
~/.config/newsman these should be handled by the program prompting you. Newsman 
will ask you if you want to use a default config that includes example values 
to help you get started.

![setup](./UserGuide/images/newsmansetup.png)

* url: This is the database url, for example mysql://root:password@localhost/newsman 
* dir: This is the directory the newsletters are stored in, it defaults to 
~/.config/newsman/newsletters/
* smtp_username: This is the username for the smtp client such as, 
example@mail.com.
* smtp_password: This is the password for the email.
* sender: Should be the same as smtp_username.
* relay: Your smtp relay for example, mail.example.com.
* interval: The interval in minutes that newsman will check if jobs need to be sent

## Usage
Newsman comes with multiple flag options which can be found with the newsman -h 
command:

* -a <Email> Adds an email to the mailing list.
* -r <Email> Removes an email from the mailing list. 
* -j <Newsletter Name> Starts a mailing job for a specified newsletter. 
* -u <Newsletter Name> Unassigns a mailing job for a specific newsletter.
* -t <Time> Time to delay the newsletter from being sent, defaults to 0 minutes.
* -e <Execute> Starts the mailing server, given true or false.
* -d <Daemon> Runs the program as a daemon, given true or false. 
* -w <Warp> Starts a warp server to listen for incoming post requests to add emails
* --debug Turns debugging information on.
* -h, --help <Help> Prints help.
* -V, --version <Version> Prints version.

## Contributing
When contributing to this project make sure to follow these steps:

1. Open an issue with your planned change
2. Make a branch for your change
3. Make your change following the projects structure
4. Open a pull request to merge your branch

These steps ensure the software doesn't suffer from feature creep and keeps the 
codebase as clean as possible. When writing the code make sure it follows Rust 
best practices as well as being placed in an appropriate file. For example if 
you write a function for handling emails make sure it's in the emails.rs file. 
If adding a new type of function make a new file in the src directory 
called function.rs, where function is whatever your function 
relates to. Any tests should be written at the bottom of the that functions file.
