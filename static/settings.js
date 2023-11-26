const Request = {
	UPDATE : 0,
	DELETE : 1
}

const HASH_ITERATIONS = 10;

var bcrypt = dcodeIO.bcrypt;
let color = document.getElementById("color");
let password = document.getElementById("password");
let description = document.getElementById("description");
let id = document.getElementById("userid").textContent;

async function update() {
	let editedAnything = false;
	let data = {}; // store the json here before pushing to server

	if (color.value != color.placeholder) {
		editedAnything = true;
		data["color"] = parseInt("0x" + (color.value.charAt(0) == '#' ? color.value.substring(1) : color.value));
	}

	if (password.value) {
		editedAnything = true;
		data["salt"] = bcrypt.genSaltSync(HASH_ITERATIONS);
		data["password"] = bcrypt.hashSync(password.value, data["salt"]);
	}

	if (description.value != description.placeholder) {
		editedAnything = true;
		data["description"] = description.value;
	}

	if (!editedAnything) {
		return;
	}

	await fetch("/settings", { method: "POST", body: Request.UPDATE + "{\"" + id + "\":" +
		JSON.stringify(data) + "}"});
	location.reload();
}

async function deleteUser() {
	if (!confirm("THERE IS NO WAY TO RECOVER YOUR DATA. ARE YOU SURE?"))
		return false;

	await fetch("/settings", { method: "POST", body: Request.DELETE + "" + id });
	console.log(`${Request.DELETE}${id}`);
	location.reload();
}