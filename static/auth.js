// this is javascript code

const Request = {
	GET_SALT : 0,
	GET_TOKEN : 1,
	REGISTER : 2,
	LOGOUT : 3
};

const COOKIE_EXPIRE = 2592000; // 30 days
const HASH_ITERATIONS = 10; // bcrypt will encode each hash saying it should iterate 10 times.
var runs = 0;
var bcrypt = dcodeIO.bcrypt;

// this function requests a token and stores it in cookies.
function requestToken() {
	const username = document.getElementById("username");
	const password = document.getElementById("password");
	const failed = document.getElementById("failed");

	if (runs > 5) {
		failed.textContent = "You have tried to login too many times!";
		return;
	}

	let redirectWhere = new URLSearchParams().get("redirectTo"); // value in url params redirectTo
	let salt;
	let hashedPassword;
	let token;

	// first obtain the salt from the server to hash with
	salt = fetch("/auth", { method: "POST", body: `${Request.GET_SALT}${username.value}` })
		.then((data) => data.json())
		.then((data) => {
			salt = data["salt"];
			// if there is no salt, quit
			if (!salt) {
				failed.textContent = `Unauthorized or invalid credentials (${++runs})`;
				return;
			}

			// hash password with the salt
			hashedPassword = bcrypt.hashSync(password.value, salt);

			// obtain the token
			fetch("/auth", { method: "POST", body: `${Request.GET_TOKEN}${JSON.stringify({
				"username" : username.value,
				"password" : hashedPassword
			})}` })
				.then(res => res.json())
				.then(data => {
					token = data["token"];

					// if there is no token, quit
					if (!token) {
						failed.textContent = `Incorrect username or password (${++runs})`;
						return;
					}

					// set the cookie and return to value of redirectTo
					redirectWhere = new URLSearchParams(window.location.search).get("redirectTo");
					console.log(redirectWhere)
					window.location.href = redirectWhere || "/";
				});
		});
}

// this function registers a new user for you, awaiting manual approval
function register() {
	let realname = document.getElementById("realname"); // concatted with description
	let prefix = document.getElementById("prefix"); // prefix to form username
	let color = document.getElementById("color"); // color of username
	let password = document.getElementById("password"); // password to be hashed
	let description = document.getElementById("description"); // their description
	let salt = bcrypt.genSaltSync(HASH_ITERATIONS); // generate the salt

	// hash the password
	let hashedPassword = bcrypt.hashSync(password.value, salt);

	// send prefix over as username, server will handle it
	fetch("/auth", { method: "POST", body: `${Request.REGISTER}${JSON.stringify({
		"username" : prefix.value,
		"password" : hashedPassword,
		"color" : Number('0x' + color.value.substring(1)),
		"description" : `I'm ${realname.value}. ${description.value}`,
		"salt" : salt
	})}` })
		.then((res) => {
		if (res.status == 403) {
			alert("Failed: The server doesn't like you.")
		} else {
			alert("Wait a day for approval.");
			location.href = "/";
		}

	});
}