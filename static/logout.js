const Request = {
	LOGOUT : 3
} 

// this function logs you out
async function logout() {
	await fetch("/auth", { method: "POST", body: `${Request.LOGOUT}` });
	window.location.href = "/";
} 
