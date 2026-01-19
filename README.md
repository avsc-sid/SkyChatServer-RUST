# SkyChat backend
this is the full stack code for skychat, codename `collo`

## Features
1. members page
2. built in chat
- automatic message loading
- reply to other people
- partially works without javascript
3. login and register (with admin approval system!)
4. dynamically generated settings
5. strong client side password hashing + rehashed on server!
- does not use basic auth compliant to RFC 2617, due to compatibility issues with apple devices

## Todo
1. Ping other users support (in chat)
2. e2e encryption
3. invite only
4. better noscript/outdated js support
6. remove bcrypt.js to fully be in public domain (wasm, maybe)
7. actual announcements page
8. projects page
9. openstreetmap integration
10. dynamic archives.html generation

## Bugs
1. an actual sanitize html function
4. parseint error & funny requests should handle correctly
5. concise the code a bit like db_get macro
6. enum split camel case

## Install
Refer to `setup.sh`

Here are the default users:
"spikestar","hey"
"leafshade","hey"

CHANGE THE PASSWORD BEFORE DEPLOYING IN PRODUCTION.

If there are any bugs that block indefinitely, try the following:

```sh
$ cargo run -r --jobs 1
```
