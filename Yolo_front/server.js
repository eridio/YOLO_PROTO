// const express = require('express')
// const app = express();

// const http = require('http').Server(app);
// const io = require('socket.io')(http);
// const port = process.env.PORT || 9000;
// app.get('/', (req, res) => {
//   res.sendFile(__dirname + '/chat.html');
// });

// app.use('/css', express.static(__dirname + '/css'));
// app.use('/js', express.static(__dirname + '/js'));
// app.use('/img', express.static(__dirname + '/img'));



// var users = new Array();

// function lookForUserSocketId(name) {
//   let i = 0;
//   for (i; i < users.length; i++) {
//     if (users[i].userName == name) {
//       return users[i].socketId;
//     }
//   }
//   return null;
// }
// function getUsersNames() {
//   let usersNames = new Array();
//   for (let i = 0; i < users.length; i++) {
//     usersNames.push(users[i].userName);
//   }
//   return usersNames;
// }

// function lookForUserName(socketId) {
//   let i = 0;
//   for (i; i < users.length; i++) {
//     if (users[i].socketId == socketId) {
//       return users[i].userName;
//     }
//   }
//   return null;
// }
// var recipientSocketId;
// var senderName;

// io.on('connection', (socket) => {
//   console.log("new  connection with id:" + socket.id);

//   socket.on("request", msg => {
//     console.log("Received message:" + msg.type);

//     let data = msg;
//     const { type, name, offer, answer, candidate, sdp } = data;



//     switch (type) {
//       //when a user tries to login

//       case "login":

//         let userName = msg.name;
//         let socketId = socket.id;
//         console.log("name :" + userName + "\n" + "id : " + socketId);
//         //send the list of users already connected
//         socket.emit("connectedUsers", getUsersNames());
//         //register new user into Users => pas sur que ca marche
//         users.push({ userName, socketId });
//         //Send a msg to all user when a new user is connected

//         socket.broadcast.emit("newUser", userName);
//         break;
//       case "offer":

//         //Get the recipient socketId
//         recipientSocketId = lookForUserSocketId(msg.name);
//         //Get the sender name
//         senderName = lookForUserName(socket.id);

//         if (senderName != null && recipientSocketId != null) {
//           console.log("User : " + senderName + " sending to " + recipientSocketId);
//           //send the offer to the recipient 
//           io.to(recipientSocketId).emit("offer", senderName);
//         }

//         break;
//       case "answer":

//         //Get the recipient socketId
//         console.log("message name " +msg.name)
//         recipientSocketId = lookForUserSocketId(msg.name);
//         //Get the sender name
//         senderName = lookForUserName(socket.id);
//         console.log("sender name"+ senderName);
//         if (senderName != null && recipientSocketId != null) {
//           //send the answer to the recipient 
//           console.log("je suis dans lif")
//           io.to(recipientSocketId).emit("answer", senderName);
//           console.log(senderName);
//         }

//         break;
//       case "sdpCaller":

//         //Get the recipient socketId
//         recipientSocketId = lookForUserSocketId(msg.name);
//         console.log(msg.dc);
//         //send the sdp to the recipient 
//         if (msg.name != null && recipientSocketId != null) {
//           io.to(recipientSocketId).emit("pcOffer", msg.sdp, msg.dc);
//         }
//         break; 

//       case "sdpCallee":
//         //Get the recipient socketId
//         recipientSocketId = lookForUserSocketId(msg.name);
//         //send the sdp to the recipient 
//         if (msg.name != null && recipientSocketId != null) {
//           io.to(recipientSocketId).emit("calleeSdp", msg.sdp);
//         }
//         break;

//       case "iceCandidateToCallee":

//         //Get the recipient socketId
//         recipientSocketId = lookForUserSocketId(msg.name);
//         //send the iceCandidate to the recipient 
//         if (msg.name != null && recipientSocketId != null) {
//           console.log(msg.candidate);
//           io.to(recipientSocketId).emit("callerIceCandidate", msg.candidate);
//         }

//         break;
//       case "iceCandidateToCaller":

//         //Get the recipient socketId
//          recipientSocketId = lookForUserSocketId(msg.name);
//         //send the iceCandidate to the recipient 
//         if (msg.name != null && recipientSocketId != null) {
//           console.log(msg.candidate);
//           io.to(recipientSocketId).emit("calleeIceCandidate", msg.candidate);
//         }

//         break;
//       case "leave":

//         recipientSocketId = lookForUserSocketId(msg.name);
//         senderName = lookForUserName(socket.id);
//         //notify the other user so he can disconnect his peer connection
//         io.to(recipientSocketId).emit("disconnection", senderName);


//         break;
//       default:
//         console.log("Error in switch");

//         break;
//     }
//   });

//   socket.on("close", function () {
//     let index = 0;
//     if (socket.id != users[index].socketId) {
//       index++;
//     }
//     else {
//       io.emit("userLeave", users[index].name);
//       delete users[index];
//     }
//   });

//   //send immediatly a feedback to the incoming connection
//   socket.send(
//     JSON.stringify({
//       type: "connect",
//       message: "Well hello there, I am a WebSocket server"
//     })
//   );
// });

// http.listen(port, () => {
//   console.log(`Socket.IO server running at http://localhost:${port}/`);
// });
const process = require('process')
process.on('SIGINT', () => {
  console.info("Interrupted")
  process.exit(0)
})

const express = require('express')
const app = express();

const http = require('http').Server(app);
const https = require('https');
const fs = require('fs')
const path = require('path')
/* const sslServer = https.createServer({
  key: fs.readFileSync(path.join(__dirname, 'cert', 'key.pem')),
  cert: fs.readFileSync(path.join(__dirname, 'cert', 'cert.pem')),
},
  app
); */
const io = require('socket.io')(http);
//const io = require('socket.io')(sslServer);
const port = process.env.PORT || 9000;
app.get('/', (req, res) => {
  res.sendFile(__dirname + '/chat.html');
});

app.use('/css', express.static(__dirname + '/css'));
app.use('/js', express.static(__dirname + '/js'));
app.use('/img', express.static(__dirname + '/img'));
app.use('/pkg', express.static(__dirname + '/pkg'));



var users = new Array();

function lookForUserSocketId(name) {
  let i = 0;
  for (i; i < users.length; i++) {
    if (users[i].userName == name) {
      return users[i].socketId;
    }
  }
  return null;
}

function getUsersNames() {
  let usersNames = new Array();
  for (let i = 0; i < users.length; i++) {
    usersNames.push(users[i].userName);
  }
  return usersNames;
}

function lookForUserName(socketId) {
  let i = 0;
  for (i; i < users.length; i++) {
    if (users[i].socketId == socketId) {
      return users[i].userName;
    }
  }
  return null;
}
var recipientSocketId;
var senderName;

io.on('connection', (socket) => {
  console.log("new  connection with id:" + socket.id);

  socket.on("request", msg => {
    console.log("Received message:" + msg.type);

    let data = msg;
    const { type, name, offer, answer, candidate, sdp, decline } = data;



    switch (type) {
      //when a user tries to login

      case "login":

        let userName = msg.name;
        let socketId = socket.id;
        console.log("name :" + userName + "\n" + "id : " + socketId);
        //send the list of users already connected
        socket.emit("connectedUsers", getUsersNames());
        //register new user into Users => pas sur que ca marche
        users.push({ userName, socketId });
        //Send a msg to all user when a new user is connected

        socket.broadcast.emit("newUser", userName);

        break;
      case "offer":

        //Get the recipient socketId
        recipientSocketId = lookForUserSocketId(msg.name);
        //Get the sender name
        senderName = lookForUserName(socket.id);

        if (senderName != null && recipientSocketId != null) {
          console.log("User : " + senderName + " sending to " + recipientSocketId);
          //send the offer to the recipient 
          io.to(recipientSocketId).emit("offer", senderName);
        }

        break;
      case "answer":

        //Get the recipient socketId
        console.log(msg.name);
        recipientSocketId = lookForUserSocketId(msg.name);
        //Get the sender name
        senderName = lookForUserName(socket.id);
        if (senderName != null && recipientSocketId != null) {
          //send the answer to the recipient 
          io.to(recipientSocketId).emit("answer", senderName);
        }

        break;
      case "decline":
        recipientSocketId = lookForUserSocketId(msg.name);
        senderName = lookForUserName(socket.id);
        if (senderName != null && recipientSocketId != null) {
          //send the answer to the recipient 
          io.to(recipientSocketId).emit("decline", senderName);
        }
        break;;
      case "sdpCaller":

        //Get the recipient socketId
        recipientSocketId = lookForUserSocketId(msg.name);
        console.log(msg.dc);
        //send the sdp to the recipient 
        if (msg.name != null && recipientSocketId != null) {
          io.to(recipientSocketId).emit("pcOffer", msg.sdp, msg.dc);
        }
        break;

      case "sdpCallee":
        //Get the recipient socketId
        recipientSocketId = lookForUserSocketId(msg.name);
        //send the sdp to the recipient 
        if (msg.name != null && recipientSocketId != null) {
          io.to(recipientSocketId).emit("calleeSdp", msg.sdp);
        }
        break;

      case "iceCandidateToCallee":

        //Get the recipient socketId
        recipientSocketId = lookForUserSocketId(msg.name);
        //send the iceCandidate to the recipient 
        if (msg.name != null && recipientSocketId != null) {
          console.log(msg.candidate);
          io.to(recipientSocketId).emit("callerIceCandidate", msg.candidate);
        }

        break;
      case "iceCandidateToCaller":

        //Get the recipient socketId
        let recipientSocketId6 = lookForUserSocketId(msg.name);
        //send the iceCandidate to the recipient 
        if (msg.name != null && recipientSocketId6 != null) {
          console.log(msg.candidate);
          io.to(recipientSocketId6).emit("calleeIceCandidate", msg.candidate);
        }

        break;

      case "quitWaiting":
        recipientSocketId = lookForUserSocketId(msg.name);
        console.log("quit wait :" + msg.name);
        senderName = lookForUserName(socket.id);
        if (recipientSocketId != null) {
          //send the answer to the recipient 
          io.to(recipientSocketId).emit("quitWaiting", senderName);
        };
        break;

      case "leave":

        recipientSocketId = lookForUserSocketId(msg.name);
        senderName = lookForUserName(socket.id);
        //notify the other user so he can disconnect his peer connection
        io.to(recipientSocketId).emit("disconnection", senderName);


        break;
      case "deconnect":
        console.log("deco deco");
        let index = 0;
        for (index = 0; index < users.length; index++) {
          if (socket.id == users[index].socketId) {
            console.log("user " + users[index].userName + " leave");
            socket.broadcast.emit("userLeave", users[index].userName);
            users.splice(index, 1);
            console.log(users);
          }
        }
        break;
      default:
        console.log("Error in switch");

        break;
    }
  });

  socket.on("disconnect", function () {
    console.log("deco deco");
    let index = 0;
    for (index = 0; index < users.length; index++) {
      if (socket.id == users[index].socketId) {
        console.log("user " + users[index].userName + " leave");
        socket.broadcast.emit("userLeave", users[index].userName);
        users.splice(index, 1);
        console.log(users);
      }
    }
  });

  //send immediatly a feedback to the incoming connection
  socket.send(
    JSON.stringify({
      type: "connect",
      message: "Well hello there, I am a WebSocket server"
    })
  );
});

http.listen(port, () => {
  console.log(`Socket.IO server running at http://localhost:${port}/`);
});

//sslServer.listen(3443, () => console.log('Secure server 🚀🔑 on port 3443'));