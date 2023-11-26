/* The type of packet sent, according to the server
 * TODO: TO BE IMPLEMENTED
 * This numeric value will always go before the JSON or other numeric values.
 * Ex: 6{"foo":"bar"} 8283428394 (notice the 8 for CLIENT_LOAD_MORE_MESSAGES first)
 */
const Server = {
    RELAYED_MESSAGE: 0,
    IS_ALIVE: 1,
    JUMP_TO_START: 2,
    LOAD_MORE_MESSAGES: 3
};

const DOMAIN = "ws://localhost:8888";

let socket = new WebSocket(DOMAIN + "/chat/ws"), // initializing the raw socket
    messageCount = 0, // amount of messages
    title = document.getElementById("title-block"), // change title when notif
    message = document.getElementById("message"), // the message field
    reconnectButton = document.getElementById("reconnect"), // the reconnecting button
    sendButton = document.getElementById("send"), // the send button
    messages = document.getElementById("messages"), // selects the entire canvas
    form = document.getElementById("form"), // message field and send button
    status = document.getElementById("statusDot"), // to change the icon when notif
	username = "test", // TODO: request username from server using token
    messageList = [],
    validated = 0,
    rem = 0,
    lastLivingServer = Date.now(),
    notifs = 0, // amount of pings you have
    focused = 1,
    viewingOld = 0,
    calibratedFocus = document.body.scrollHeight - window.scrollY,
    replyID = "",
    replyName = "",
    savedReply = "",
    deltaX = 0

setupConnection();

window.scrollTo(0, document.body.scrollHeight);

if (document.body.scrollHeight - window.scrollY < calibratedFocus + 40)
    window.scrollTo(0, document.body.scrollHeight)

// when the user focuses on the window
window.addEventListener("focus", _ => {
    focused = 1;
    notifs = 0;
    status.href = "logo.png";
    title.textContent = "SkyChat";
    // if the user minimized the tab for less than 30 seconds
    if (Date.now() - lastLivingServer > 1000 * 30)
        reconnect();
});

// when the tab is switched
window.addEventListener("blur", _ => {
    focused = 0;
    lastLivingServer = Date.now();
});

window.addEventListener("touchstart", e => {
    deltaX = e.changedTouches[0].clientX;
});

window.addEventListener("touchend", e => {
    if (e.changedTouches[0].clientX - deltaX > 50) {
        if (replyID.length) {
            message.placeholder = "Replying to " + replyName
            savedReply = replyID;
        } else {
            message.placeholder = "Enter a message"
        }
        replyID = ""
    }
});

window.addEventListener("mouseup", e => {
    if (replyID.length) {
        message.placeholder = "Replying to " + replyName
        message.focus()
        savedReply = replyID
    } else {
        message.placeholder = "Enter a message"
    }
    replyID = ""
});

// this function is for when the connection bugs out
function reconnect() {
    socket.close();
    reconnectButton.textContent = "Connecting...";
    socket = new WebSocket(DOMAIN + "/chat/ws");
    setupConnection();
}

// if typing on the keyboard, then focus
document.body.addEventListener("keydown", e => {
    message.focus();
});


// all the socket listeners nested once the connection is established
function setupConnection() {
    socket.addEventListener("close", (event) => {
        console.log('disconnected')
    });

    socket.addEventListener("open", (event) => {
        console.log('connected');
    });

    socket.addEventListener("message", (event) => {
        switch (event.data) {
            case Server.RELAYED_MESSAGE:
                postMessage(m);
                break;
            case Server.BACKTRACE:
                groupMessages(ms);
                break;
            case Server.IS_ALIVE:
                // handling of server's response to client's ping
                lastLivingServer = Date.now();
                console.log('server was alive at', formatDate(Date.now()));
                break;
            case Server.UPDATE_TO_START:
                updateToStart();
                break;

        }
    });
}

// when the message is submmited by the user
form.addEventListener('submit', function(e) {
    window.scrollTo(0, document.body.scrollHeight);
    // clean up old messages that the user doesn't need rendered
    // if the screen size is smaller than 800x600, delete 20 messages else delete 50
    messageList = messageList.slice(Math.max(0,
            messageList.length - (((window.innerWidth <= 800) && (window.innerHeight <= 600)) ? 20 : 50)),
        messageList.length)

    // TODO: what is this
    calibratedFocus = document.body.scrollHeight - window.scrollY;
    e.preventDefault();

    // change the placeholder back in case of a reply
    message.placeholder = "Enter a message";
    rem = 0;
    send();
});


function send() {
    if (message.value.length) {
        // add an initial greyed out message.
        addMessage({
            message: format(message.value),
            timestamp: Date.now(),
        });
        composeHTML();

        window.scrollTo(0, document.body.scrollHeight);
        let a = 0;
        if (savedReply) {
            console.log('reply detected');
            a = messageList.filter(x => x.id == savedReply);
            if (a.length) {
                let rt = a[0].message;
                rt = rt.split("</font></span>  ");
                if (rt.length > 1) rt = rt[1]
                else rt = rt[0]
                a = {
                    message: rt,
                    timestamp: a[0].timestamp,
                    color: a[0].color,
                    name: a[0].name
                }
            }
        }
        socket.send(Server.CLIENT_RELAY_MESSAGE + JSON.stringify({
            message: message.value,
            timestamp: Date.now(),
            reply: a
        }));
        messageCount++;
        savedReply = "";
        message.value = "";
    }
}
const postMessage = m => {
    if (viewingOld) return
    let a = messageList.filter(x => x.id == m.id)
    if (a.length) {
        a[0].message = format(m.message)
        a[0].color = m.color
        composeHTML()
    } else {
        addMessage({
            name: m.name,
            id: m.id,
            message: format(m.message),
            timestamp: m.timestamp,
            color: m.color
        })
        if (!focused) {
            if (m.message.toLowerCase().includes("@" + formatName(username).toLowerCase()) ||
                m.message.includes(formatName(username) + "</font>")) notifs++
            title.textContent = "(" + notifs + ") SkyChat";
            status.href = "logo-notif.png"
        }
        messageList = messageList.slice(Math.max(0, messageList.length -
            (((window.innerWidth <= 800) && (window.innerHeight <= 600)) ? 20 : 50)), messageList.length)
        composeHTML()
    }
    if (document.body.scrollHeight - window.scrollY < calibratedFocus + 600 || Date.now() - validated < 3000)
        window.scrollTo(0, document.body.scrollHeight)
}

const groupMessages = ms => {
    let parsed = JSON.parse(ms)
    for (let i = 0; i < parsed.length; i++) {
        if (!messageList.filter(x => x.id == parsed[i].id).length) addMessage({
            name: parsed[i].name,
            id: parsed[i].id,
            message: format(parsed[i].message),
            timestamp: parsed[i].timestamp,
            color: parsed[i].color
        })
    }
    if (messageList.length > 70) {
        messageList = messageList.slice(0, 70)
        viewingOld = 1
    }
    composeHTML()
}
const shiftToStart = _ => {
    socket.send(Server.CLIENT_JUMP_TO_START);
}
const updateToStart = ms => {
    viewingOld = 0
    messageList = JSON.parse(ms)
    composeHTML()
}
const loadMoreMessages = _ => {
    message.focus()
    // this sends a message with the following concatnated to form something like "8202307183290"
    socket.send(Server.CLIENT_LOAD_MORE_MESSAGES + "" + messageList[0].timestamp)
}
const addMessage = m => {
    for (let i = messageList.length - 1; i > -1; i--) {
        if (messageList[i].timestamp < m.timestamp) {
            messageList.splice(i + 1, 0, m)
            return
        }
    }
    messageList.splice(0, 0, m)
}
const composeHTML = _ => {
    messageList = messageList.filter(x => !x.message.startsWith("!"))
    let b = 0
    if (messages.innerHTML == "") b = 1
    let a = `
       <font color="#FFF">helo groundgang fr</span><br>
       <button style="font-size:25px" onclick="loadMoreMessages()">Load more messages</button><br><br>` //ignore this its bc im bad at css so idk how to indent things n stuff bad html pactice tbh
    messages.innerHTML = a
    if (messageList.length < 17) {
        a = `
    <br><br><br><br><br><br><br><br><br><br>
    <br><br><br><br><br><br><br><br><br><br>
    <br><br><br><br><br><br><br><br><br><br>
	<br><br><br><br><br><br><br><br><br><br>
    <br><br><br><br><br><br><br><br><br><br>` + a
    }
    for (let i = 0; i < messageList.length; i++) {
        a = document.createElement("li")
        a.id = messageList[i].id
        let c = format(messageList[i].message)
        let namething = formatName(messageList[i].name)
        if (namething == formatName(username))
            namething = "<u>" + namething + "</u>"
        c = c.replaceAll('@' + formatName(username).toLowerCase(), `
			</font><font color="#eed49f">
			<span style="background-color:#55F;padding:2px;border-radius:5px">
			@​${formatName(username).toLowerCase()}</span></font><font color="#cad3f5">`)
        a.innerHTML = `<font color="#b8c0e0">
	   		<button style="font-size:15px" onmousedown="replyID='${messageList[i].id}';replyName='${messageList[i].name}'" ontouchstart="replyID='${messageList[i].id}';replyName='${messageList[i].name}'" title="${formatDate(messageList[i].timestamp, 1)}">${formatDate(messageList[i].timestamp)}</button> </font><font color="${messageList[i].color}">${namething}</font><font color="#cad3f5">: ${c}</font>`
        messages.appendChild(a)
    }
    if (viewingOld) {
        a = document.createElement("li")
        a.id = 'unupdatedMessageAlertViewHandler'
        a.innerHTML = `<font color="#b8c0e0"><button style="font-size:15px" onmousedown="shiftToStart()" ontouchstart="shiftToStart()">You're viewing older messages - jump to start</button></font>`
        messages.appendChild(a)
    }
    messages.innerHTML += "<br>"
    if (document.body.scrollHeight - window.scrollY < calibratedFocus + 40 || b) window.scrollTo(0, document.body.scrollHeight)
}

//format incoming markdown text to html
function format(m) {
    m = m.replaceAll("@​", "<atsymbol>")
    m = m.replaceAll('<span style="background-color:#494d64;other-thing:' + formatName(username), '<span style="background-color:#55F')
    m = m.replaceAll("<atsymbol>", "@​")
    let a = m.split(" ")
    for (let i = 0; i < a.length; i++) {
        if (a[i].startsWith('https://') || a[i].startsWith('http://'))
			a[i] = '</font><a href="' + a[i] + '" target="_blank">' + a[i] + '</a><font color="#DDD">'

        //fixed
        //but i need to fix the other thing somewhere
        //otherwise it wont load
    }
    m = a.join(' ')
    a = null
    return m
}

let noextras = /[^A-Za-z0-9]/g;

function formatName(name) {
    return name.replace(noextras, "");
}

function formatDate(d) {
    let a = new Date()
    a.setTime(d)
    return a.toLocaleTimeString().replaceAll("AM", "").replaceAll("PM", "")
}