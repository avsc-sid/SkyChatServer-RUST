// these represent the first byte of the messages that will be sent, NOT RECIEVED
const Messages = {
    RELAY_MESSAGE: '0',
    LOAD_MORE_MESSAGES: '1'
};

// dynamically assign the websocket url
const websocketUrl = ((window.location.protocol === "https:" ? "wss://" :
        "ws://") +
    window.location.host + "/chat/ws");

let socket = new WebSocket(websocketUrl);
let messages = document.getElementById(
"messageContainer"); // selects where messages are supposed to go
let messageInput = document.getElementById(
"message"); // selects message field so 'Enter' can be a keypress
let send = document.getElementById(
"send"); // selects send button to emulate key press
let reconnectButton = document.getElementById(
"reconnect"); // selects reconnect to enable disable button as needed
let replyTo = null;

// timestamp of last message
let previous = new Date(1970, 1, 1);

// timestamps to local time
for (let i = 0; i < messages.children.length; i++) {
    let text = messages.children[i].children[0].textContent;
    let current = new Date(parseInt(text) * 1000);

    // print date if last message was more than a day ago
    if (previous.getDate() < current.getDate() ||
        previous.getMonth() < current.getMonth() ||
        previous.getFullYear() < current.getFullYear()) {

        let date = document.createElement("i");
        date.setAttribute("unix", text);
        date.textContent = current.toLocaleDateString();

        messages.insertBefore(date, messages.children[i]);
        i++;
        previous = current;
    }

    messages.children[i].children[0].textContent = current.toLocaleTimeString();
}

// scroll to the bottom, see latest messages
window.scrollTo(0, document.body.scrollHeight);

setupConnection();

// colors in the database are stored as integers so converting to rgb is necessary
function toColor(num) {
    return "#" + ((num) >>> 0).toString(16).slice(-6);
}

function attemptReply(msgId, msgAuthor) {
    replyTo = msgId;
    messageInput.focus();
    messageInput.select();
    messageInput.placeholder = "Replying to " + msgAuthor;
}

function messageToHTML(message) {
    let messageElement = document.createElement("li");
    messageElement.id = message["id"];

    // Date class interprets the timestamp as milliseconds
    messageElement.innerHTML = `
<button class="timestamp" onclick="attemptReply(${messageElement.id}, '${message["authorName"]}')">
${new Date(message["timestamp"] * 1000).toLocaleTimeString()}
</button>
<span class="author" style="color: ${toColor(message["authorColor"])}" data="${message["authorId"]}">
${message["authorName"]}</span>`

	if (message["repliedTo"]) {
		messageElement.innerHTML += `<i>replied to ${message["repliedTo"]}</i> `;
	}

	messageElement.innerHTML += `: ${message["content"]}`;

    return messageElement;
}

function setupConnection() {
    socket = new WebSocket(websocketUrl);

    socket.onopen = () => {
        reconnectButton.disabled = false;
        reconnectButton.textContent = "Reconnect";

    };

    socket.onmessage = (event) => {
        switch (event.data.charAt(0)) {
            case Messages.RELAY_MESSAGE:
                const message = JSON.parse(event.data.substring(1));

                // print date if last message was more than a day ago
                let current = new Date(message["timestamp"] * 1000);
                if (previous.getDate() < current.getDate() ||
                    previous.getMonth() < current.getMonth() ||
                    previous.getFullYear() < current.getFullYear()) {

                    let date = document.createElement("i");
                    date.textContent = current.toLocaleDateString();
                    date.setAttribute("unix", message["timestamp"]);

                    messages.appendChild(date);
                    previous = current;
                }

                // append to messages
                messages.appendChild(messageToHTML(message));

                // scroll to the latest message
                window.scrollTo(0, document.body.scrollHeight);
                break;
            case Messages.LOAD_MORE_MESSAGES:
                const oldMessages = JSON.parse(event.data.substring(1));
                for (let i = 0; i < oldMessages.length; i++) {
                    // messages.children[0] is a date
                    let date = messages.children[0];

                    for (let j = 1; j < messages.children.length; j++) {
                        let currentDate = messages.children[i].getAttribute(
                            "unix");

                        if (currentDate && currentDate > date) {
                            date = currentDate;
                        }

                        if (oldMessages[i]["id"] < messages.children[j]
                            .id) {
                            let dateObject = new Date(parseInt(date));
                            let currentDateObject = new Date(parseInt(
                                oldMessages[i]["timestamp"]));

                            if (dateObject.getDate() < currentDateObject
                                .getDate() ||
                                dateObject.getMonth() < currentDateObject
                                .getMonth() ||
                                dateObject.getFullYear() < currentDateObject
                                .getFullYear()) {

                                let dateElement = document.createElement(
                                    "i");
                                dateElement.setAttribute("unix", date);
                                dateElement.textContent = currentDateObject
                                    .toLocaleDateString();

                                messages.insertBefore(dateElement, messages
                                    .children[i]);
                                i++;
                                date = oldMessages[i]["timestamp"];
                            }

                            messages.insertBefore(
                                messageToHTML(oldMessages[i]), messages
                                .children[j]);
                            break;

                        }
                    }
                }

                break;
            default:
                console.log("unknown event: '" + event.data + "'");
                break;
        }
    };

    socket.onclose = () => {
        // recreate websocket
        ws = null;
        setTimeout(setupConnection, 5000);
    };
}

send.addEventListener("click", () => {
    if (messageInput.value != "") {
        if (socket?.readyState !== WebSocket.OPEN) {
            alert("Connection not ready.");
            return;
        }

        socket.send(Messages.RELAY_MESSAGE + JSON.stringify({
            content: messageInput.value,
            repliedTo: replyTo
        }));
        messageInput.value = "";
        messageInput.placeholder = "Enter a message";
        replyTo = null;
    }

    // when the button acts as a jump to bottom
    window.scrollTo(0, document.body.scrollHeight);
});

reconnectButton.addEventListener("click", () => {
    reconnectButton.textContent = "Connecting...";
    reconnectButton.disabled = true;
    // refer to onclose event handler
    socket.close();
});

// listen for keypresses so we can record 'Enter'
messageInput.addEventListener("keypress", function(event) {
    if (event.key === "Enter") {
        event.preventDefault(); // cancel default operation
        send.click();
    }
});

// automatic focus on message field if you start typing
document.body.addEventListener("keydown", function(event) {
    messageInput.focus();
});

// if you scroll to top, send a load more messages event.
window.onscroll = function() {

    // if at top and first message isn't already loaded
    if (document.scrollingElement.scrollTop == 0 && messages.children[1]
        .id > 1) {
        if (socket?.readyState !== WebSocket.OPEN) {
            return;
        }

        socket.send(Messages.LOAD_MORE_MESSAGES + messages.children[1].id -
            1);
    }

    // if page is just not scrolled to bottom
    else if ((window.innerHeight + window.pageYOffset) < scroll
        .scrollHeight - 500) {
        send.textContent = "Down/Send";
    } else {
        send.textContent = "Send";
    }
}

// TODO: button as replacement
// if (! "scroll" in window) { }