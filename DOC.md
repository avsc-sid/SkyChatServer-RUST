# SkyClan API
Here is the official documentation of the SkyClan API. This is subject to change.
The entire documentation is written assuming you have some knowledge of shell script.

## Logging in
Logging in is a two-step process, since the passwords need to be hashed locally before being sent to the server.

First, you request the salt used for hashing your password from the server. A salt is a string concatted with your password to increase enthropy, but over here it is just a parameter for a local bcrypt hash. To do this, we access the API endpoint /auth:

`curl https://skyclan.us.to/auth -d "0spikestar"`

This command sends the data "0spikestar" to /auth (NOTE THAT CURL AUTOMATICALLY MAKES THIS A POST REQUEST). 0 is the first byte denoting what kind of a request it is, in our case this says that we are asking for a salt. "spikestar" is an example username used, but really you can request the salt for any username.

This returns JSON data in the form of:
```
{
    "salt" : "...",
    "success" : 200
}
```

Note that "..." will be replaced with an actual UTF-8 encoded salt.

For the hashing, I wrote a command "bcrypt" so you can take full advantage of the fast hashing algorithm in machine code form. [Code](https://rslp.org/~moe/git/bcrypt/log.html). You can use a bcrypt implementation in python or javascript or such if it makes it easier.

Now, just run `bcrypt -s "salt" -t "password"` to obtain your hashed password.
