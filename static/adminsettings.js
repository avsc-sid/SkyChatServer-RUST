const Table = {
	DELETE_BUTTON : 0,
	USERNAME : 1,
	COLOR : 2,
	PASSWORD : 3,
	SALT : 4,
	STATUS : 5,
	DESCRIPTION : 6
} 

const Request = {
	UPDATE : 0,
	DELETE : 1,
	CREATE : 2
} 

const HASH_ITERATIONS = 10;

var bcrypt = dcodeIO.bcrypt;
let userTable = document.getElementById("userTable");
let bigDescription = document.getElementById("bigDescription");
let usersForDesc = document.getElementById("usersForDesc");

// for createUser()
let name = document.getElementById("name"); // username
let color = document.getElementById("color"); // color of username
let password = document.getElementById("password"); // password to be hashed
let userStatus = document.getElementById("status"); // status
let description = document.getElementById("description"); // their description

// hide the salt column using js cuz noscript is weird
for (var i = 0; i < userTable.rows.length - 1; i++) {
	userTable.rows[i].children[4].hidden = true;
} 

async function update() {
	let editedAnything = false;
	let data = {}; // store the json here before pushing to server

	// iterate, but skip over column title and delete button
	// and last row to make new
	for (var i = 1; i < userTable.rows.length - 1; i++) {
		// delete button contains the ID
		let id = userTable.rows[i].cells[Table.DELETE_BUTTON].firstChild.id;
		// store the value in here as cache
		let value;
		let user = {}; // another json object per user

		if ((value  = userTable.rows[i].cells[Table.USERNAME].firstChild.value) != "") {
			editedAnything = true;
			user["username"] = value;
		} 

		if ((value  = userTable.rows[i].cells[Table.COLOR].firstChild.value) != "") {
			value = value.charAt(0) == '#' ? value.substring(1) : value;
			if (value != userTable.rows[i].cells[Table.COLOR].firstChild.placeholder) {
				editedAnything = true;
				user["color"] = parseInt("0x" + value);
			} 
		} 

		if ((value  = userTable.rows[i].cells[Table.PASSWORD].firstChild.value) != "") { editedAnything = true; // since this is the password, we need to take some extra care and do some hashing.
			user["salt"] = bcrypt.genSaltSync(HASH_ITERATIONS);
			user["password"] = bcrypt.hashSync(value, user["salt"]);
		} 

		if ((value  = userTable.rows[i].cells[Table.STATUS].firstChild.value) != "" && !isNaN(value)) {
			editedAnything = true;
			user["status"] = parseInt(value);
		} 

		if ((value  = userTable.rows[i].cells[Table.DESCRIPTION].children[0].value) != "" && value != 
						userTable.rows[i].cells[Table.DESCRIPTION].children[0].placeholder) {
			editedAnything = true;
			user["description"] = value;
		} 

		data[id] = user;
	} 

	if (bigDescription && bigDescription.value && bigDescription.value != bigDescription.placeholder) {
		data[usersForDesc.options[usersForDesc.selectedIndex].id]["description"] = bigDescription.value;
		editedAnything = true;
	} 

	if (!editedAnything) {
		return;
	} 

	await fetch("/settings", { method: "POST", body: Request.UPDATE + JSON.stringify(data)});
	location.reload();
} 

async function deleteUser(id) {
	await fetch("/settings", { method: "POST", body: `${Request.DELETE}${id}` });
	location.reload();
} 

async function createUser() {
	let salt = bcrypt.genSaltSync(HASH_ITERATIONS); // generate the salt

	intColor = parseInt("0x" + (color.value.charAt(0) == '#' ? color.value.substring(1) : color.value));

	if (!name.value || isNaN(intColor) || isNaN(userStatus.value)) {
		console.log("invalid");
		return;
	} 

	// hash the password
	let hashedPassword = bcrypt.hashSync(password.value ? password.value : "default", salt);

	await fetch("/settings", { method: "POST", body: Request.CREATE + JSON.stringify({
		"username" : name.value, 
		"password" : hashedPassword,
		"color" : intColor,
		"description" : description.value,
		"status" : parseInt(userStatus.value),
		"salt" : salt
	}) });
	location.reload();
}

function clearDescription(iden) {
	for (var i = 1; i < userTable.rows.length - 1; i++) {
		if (userTable.rows[i].children[Table.DELETE_BUTTON].firstChild.id == iden) {
			userTable.rows[i].children[Table.DESCRIPTION].children[0].value = "";
		} 
	} 

	// also clear bigDescription in case
	bigDescription.value = "";
} 

usersForDesc.onchange = function (event) {
	updateBigDesc(event.target.options[event.target.selectedIndex].id);
};
function updateBigDesc(iden) {
	//TODO: what
	for (var i = 1; i < userTable.rows.length - 1; i++) {
		if (userTable.rows[i].children[Table.DELETE_BUTTON].firstChild.id == iden) {

			bigDescription.value = userTable.rows[i].children[Table.DESCRIPTION].children[0].placeholder;
			bigDescription.placeholder = bigDescription.value;
		} 
	} 
} 
