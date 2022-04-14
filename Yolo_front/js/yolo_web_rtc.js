var socket = io();
var userName = "patoche" + Math.random();
        

//flo
function delay(n){
    return new Promise(function(resolve){
        setTimeout(resolve,n);
    });
    }

window.onload = oninit();
async function oninit(){
    await delay(1000)
    get_your_name();
    
}

function get_your_name() {
    let xhr = new XMLHttpRequest();
    let cookie_to_send = document.cookie.substring(4,200);
    xhr.open("GET", `https://127.0.0.1:4201/get_your_name`, true);
    xhr.setRequestHeader( "Authorization","Bearer " + cookie_to_send );
    xhr.onload = () => {
        if (xhr.readyState === 4) {
            if (xhr.status === 200) {
                console.log(xhr.statusText);
                parseData_get_your_name(xhr.responseText);
            } else {
                console.error(xhr.statusText);
            }
        }
    };
    xhr.onerror = () => {
        console.error(xhr.statusText);
    };
    xhr.send(null);
}
function parseData_get_your_name(response) {
    let json_received = JSON.parse(response);
    console.log(json_received)
    username = json_received.full_name; 
    userName = username
    socket.emit("request", msg = { type: "login", name: userName });
}

//############################################## SIGNAL_PROTOCOL PART ###############""

//############################################## SIGNAL_PROTOCOL PART ###############""
/*
var firstName = ...
var lastName = ...
*/


var userList = new Array();
var callee;
var options = {
    iceServers: [
        {
            urls: "stun:stun.l.google.com:19302"
        }
    ]
}
var caller;
var pcCaller;
var pcCallee;
var dataChannel;


//envoie msg 

let SendButton = document.getElementById("button");
SendButton.addEventListener("click", e => {
    let divMessage = document.getElementById("messageInput");
    sendMessage(dataChannel, divMessage.value);
    divMessage.value = "";
});





function messageReceive(dataChannel) {
    dataChannel.addEventListener("message", event => {
        console.log("Message received : " + event.data);
        let divMsg = document.getElementById("conversation");
        divMsg.innerHTML += `
        <div class="row message-body">
            <div class="col-sm-12 message-main-receiver">
                <div class="receiver">
                    <div class="message-text">
                            ${event.data}
                            </div>
                    </div>
            </div>
        </div>
        `;
    });
}
function sendMessage(dataChannel, message) {
    if (dataChannel.readyState == "open") {
        dataChannel.send(message);
        let divMsg = document.getElementById("conversation");
        divMsg.innerHTML += `
        <div class="row message-body">
            <div class="col-sm-12 message-main-sender">
                <div class="sender">
                    <div class="message-text">
                    ${message}
                    </div>
                </div>
            </div>
        </div>
        `;
    }
};
function displayOffer(senderName) {
    let divModal = document.getElementById("modal");
    divModal.innerHTML += `<div id="offer" class="modal">
        <div class="modal-dialog">
          <div class="modal-content">
            <header class="modalContainer"> Offer By ${senderName}</header>
            <button class=acceptButton id=${senderName}>Accept offer send by:${senderName}</button>          
          </div >
        `;
}

/*
    const stream = navigator.mediaDevices.getUserMedia({ audio: true });
 
    var pcCaller = new RTCPeerConnection(options);;
    const audioTrack = stream.getTracks().array.forEach(track => {
     pcCaller.addTrack(track, stream);
    });;
 
*/


var cells = document.getElementById("cells");
socket.on("getUsers", users => {
    for (let i = 0; i < users.length; i++) {
        addUsers(users[i]);
    }
});

function addUsers(user) {
    userList.push(user);

    let id = userList.length;
    //Add to the list of user on the UI 
    let contactList = document.getElementById("friends");
    contactList.innerHTML += `
        <div class="row sideBar-body">
            <div class="col-sm-3 col-xs-3 sideBar-avatar">
                <div class="avatar-icon">
                    <img src="img/man-2-512.png">
                </div>
            </div>
            <div class="col-sm-9 col-xs-9 sideBar-main">
                <div class="row">
                    <div class="col-sm-8 col-xs-8 sideBar-name">
                        <span class="name-meta">${user}
                        </span>
                         <p><button class="offerButton" id="${user}">Make an offer</button></p>
                    </div>
                </div>
            </div>
        </div>
        `;
 //   console.log(contactList);

};

socket.on("newUser", user => {
    addUsers(user);
    console.log(user);
});

socket.on("connectedUsers", usersAlreadyConnected => {
    for (let i = 0; i < usersAlreadyConnected.length; i++) {
        addUsers(usersAlreadyConnected[i]);
    }
});

//'''''''''''''''''''''''''''Caller side''''''''''''''''''''''''''''''''//

// callee accepted the offer 
socket.on("answer", async receiverName => {
    //création sdp et envoi de sdp puis de ICE
    //côté caller
    callee = receiverName;
    console.log("Connexion accepté de :" + receiverName);

    //Creating the caller peer connection and his sdp
    if (pcCaller == undefined) {
        pcCaller = new RTCPeerConnection();
        dataChannel = pcCaller.createDataChannel("dataChannel");
        //console.log(dataChannel);
    }


    pcCaller.addEventListener('connectionstatechange', event => {
        console.log("connection ?")
        if (pcCaller.connectionState === 'connected') {
            // Peers connected!
            messageReceive(dataChannel);
            console.log("GGWP peers connected!");
        }
    });

    var CallerSdp = await pcCaller.createOffer({ iceRestart: true });
    //Sending the caller sdp to the callee  
    socket.emit("request", { type: "sdpCaller", name: receiverName, sdp: CallerSdp, dc: dataChannel })
    //Setting the caller (his) local description
    await pcCaller.setLocalDescription(CallerSdp);
});
//Waiting for callee sdp 

// when callee send spd info
socket.on("calleeSdp", async calleeSdp => {
    //Caller set callee description
    const remoteDesc = new RTCSessionDescription(calleeSdp);
    console.log(pcCaller.iceGatheringState);
    await pcCaller.setRemoteDescription(remoteDesc);
    console.log("callee descripiton set");
    console.log(remoteDesc);

    //navigator.mediaDevices.getUserMedia({ audio: true, video: true });
    //pcCaller.addStream(localStream);

    //Then 
    //Caller listen to his peerconnection for some icecandidate and when one is found the caller send it to the callee 

    console.log(pcCaller.iceGatheringState);



    pcCaller.addEventListener('icecandidate', event => {

        if (event.candidate != null) {
            console.log("ice candidate found");
            console.log(event.candidate);
            socket.emit("request", { type: 'iceCandidateToCallee', name: callee, candidate: event.candidate });
        }
    });

    dataChannel.addEventListener("open", ev => {
        const readyState = dataChannel.readyState;
        console.log("Send channel state is: " + readyState);
        sendMessage(dataChannel, "coucou bro 2");
    });
    dataChannel.addEventListener("error", ev => {
        console.log(ev);
    });




    // Listen for connectionstatechange on the local RTCPeerConnection



});



socket.on('calleeIceCandidate', async calleeIceCandidate => {
    console.log(calleeIceCandidate);
    if (calleeIceCandidate) {
        //Try to add the caller ice candidate 
        try {
            await pcCaller.addIceCandidate(calleeIceCandidate);
            console.log("callee ice cadidate added");
        } catch (e) {
            console.error('Error adding received ice candidate', e);
        }
    }
});


//-----------------------------Callee side----------------------------------//

// callee received an offer from the caller 
socket.on("offer", senderName => {
    console.log("Demande de connexion de:" + senderName);
    displayOffer(senderName);
    //cells.innerHTML += `<button class=acceptButton id = ${senderName}> Accept offer send by:${senderName}</button >`;
});


//receiving a peerConnection offer from the caller (send by the signaling server)
socket.on("pcOffer", async (callerSdp, dc) => {
    //creating the callee peer connection
    pcCallee = new RTCPeerConnection();
    // dataChannel = dc;
    // console.log(dc);
    pcCallee.addEventListener("datachannel", ev => {
        dataChannel = ev.channel;

        console.log("datachannel ? : ");
        console.log(dataChannel);

        sendMessage(dataChannel, "coucou bro");
        messageReceive(dataChannel);

    }, false);
    //messageReceive(dataChannel);
    //Setting the caller sdp description
    await pcCallee.setRemoteDescription(callerSdp);
    console.log("caller description set:");
    console.log(callerSdp);

    pcCallee.addEventListener('icecandidate', event => {

        if (event.candidate != null) {
            console.log("ice candidate found");
            console.log(event.candidate);
            socket.emit("request", { type: 'iceCandidateToCaller', name: callee, candidate: event.candidate });
        }
    });
    //Creating the callee sdp answer 
    var calleeSdp = await pcCallee.createAnswer();
    pcCallee.setLocalDescription(calleeSdp);

    //Sending the callee sdp to the caller 
    socket.emit("request", { type: "sdpCallee", name: caller, sdp: calleeSdp })

});

//Signaling server emiting caller ice candidate 
socket.on('callerIceCandidate', async callerIceCandidate => {
    console.log(callerIceCandidate);
    if (callerIceCandidate) {
        //Try to add the caller ice candidate 
        try {
            await pcCallee.addIceCandidate(callerIceCandidate);
            console.log("caller ice cadidate added");
        } catch (e) {
            console.error('Error adding received ice candidate', e);
        }
    }

    // searching for callee candidate 

});




/////////////////////////////////////////////////////////////////////////////////////////

socket.on("disconnection", senderName2 => {

});

const acceptButton = document.getElementsByClassName("Accept");
document.addEventListener('click', function (e) {
    if (e.target && e.target.className == 'offerButton') {
        //Caller sending his offer to the callee 
        socket.emit("request", { type: "offer", name: e.target.id });
        console.log(e.target.id);
    }
    if (e.target && e.target.className == 'acceptButton') {
        console.log(e.target);
        let divModal = document.getElementById("offer").style.display = "none";
        //callee accepted the offer and sending back his answer to the caller 
        socket.emit("request", { type: "answer", name: e.target.id });
        console.log(e.target.id);
        caller = e.target.id;
    }

});


