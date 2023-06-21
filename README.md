# Newsman
A emailing server written for sending newsletters to users of a service

## Project Status
These are the current function that need to be written

| Function | Status |
|---|---|
| Add email to database | :heavy_check_mark: |
| Remove email from database | :heavy_check_mark: |
| Start mailing job | :heavy_check_mark: |
| Execute mailing job | :heavy_check_mark: |
| Start daemon | :heavy_check_mark: |
| Stop daemon | :heavy_check_mark: |
| Main function | :heavy_check_mark: |

## Setup
You will also need to make the database which can be found in mailing_list.sql, 
at the moment only mysql databases work with the program.

```
$ mariadb -u root -p < mailing_list.sql
```
