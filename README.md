# Newsman
A emailing server written for sending newsletters to users of a service

## Requirements

![REQUIREMENTS](/.diagrams/diagram.png)

## Project Status
These are the current function that need to be written

| Function | Status |
|---|---|
| Add email to database | - [ ] |
| Remove email from database | - [ ] |
| Start mailing job | - [ ] |
| Execute mailing job | - [ ] |
| Start daemon | - [ ] |
| Stop daemon | - [ ] |
| Main function | - [ ] |

## Prepare for use 
In the projects root you need to make a .env file with DATABASE_URL and NEWSLETTER_DIR
set before the program works.

```
$ touch .env
$ echo "DATABASE_URL=database_url\nNEWSLETTER_DIR=newsletter_dir\n
SMTP_USERNAME=smtp_username\nSMTP_PASSWORD=smtp_password\nSENDER=sender\n
RELAY=relay" >> .env

```
