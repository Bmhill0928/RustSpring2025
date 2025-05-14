## Overview

The project created is a Rust project that can concurrently check the status(i.e. does the website work/run) of any desired website.

This project can check one or multiple urls from a command line argument or a file with an adjustable amount of retries.

When this program is ran a status.json file is created and information will be displayed for whether or not the website passed the check or failed and will show the information tied to that specific website.

### Below is an example of how the program is ran:

- cargo run -- --file websites.txt --workers 8 --timeout 5 --retries 3

- cargo run https://google.com --workers 4 --timeout 5 --retries 2

## Example Outputs(json file and terminal)

### Command Line
 - Starting to process URLs with 4 workers...
Successfully processed: https://google.com
Processing completed in 156.876691ms

### Command Line
 - Starting to process URLs with 4 workers...
Failed to process https://nonexistentwebsite.com: Error: error sending request for url (https://nonexistentwebsite.com/)
Processing completed in 1.162905644s

### File
 - Starting to process URLs with 8 workers...
Successfully processed: https://google.com
Successfully processed: https://rust-lang.org
Failed to process https://nonexistentwebsite.com: Error: error sending request for url (https://nonexistentwebsite.com/)
Processing completed in 2.101641964s

### File
 - [
{"url": "https://google.com", "action_status": 200, "response_time": 122, "timestamp": "SystemTime { tv_sec: 1747228961, tv_nsec: 863298312 }"},
{"url": "https://rust-lang.org", "action_status": 200, "response_time": 417, "timestamp": "SystemTime { tv_sec: 1747228962, tv_nsec: 158468232 }"},
{"url": "https://nonexistentwebsite.com", "action_status": "Failed to process https://nonexistentwebsite.com: Error: error sending request for url (https://nonexistentwebsite.com/)", "response_time": 2071, "timestamp": "SystemTime { tv_sec: 1747228963, tv_nsec: 808267041 }"}
]

## Notes

 - This project does utilize std::thread, Mutex, and Arc in order to use concurrency

 - HTTP requests are made using the helper crate reqwest.

 - The output will show the status of the website and will also provide the response time for checking the website.

 - When the program is a status.json file is made and updated with the information of the website(s) ran.

 - Only one helper crate was used, so things such as the time in the status.json may not look right, but Duration was used to replace as best we could.


