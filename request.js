const url = 'https://127.0.0.1:4201';

window.onload = oninit();
function oninit(){
    //GET TES AMIS 
    
    getyourfriend()
}




function getyourfriend() {
let xhr = new XMLHttpRequest();
let cookie_to_send = document.cookie.substring(4,200);
console.log(cookie_to_send);
xhr.open("GET", url + `/get_your_friend`, true);
xhr.setRequestHeader( "Authorization","Bearer " + cookie_to_send );
xhr.onload = () => {
    if (xhr.readyState === 4) {
        if (xhr.status === 200) {
            console.log(xhr.statusText);
            parseData_for_getFriends(xhr.responseText);
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
function parseData_for_getFriends(response) {
search_response = JSON.parse(response)
console.log("friends : ")
console.log(search_response)
for (var i = 0; i < search_response.length; i++) {
    

    document.getElementById("demo").innerHTML += (search_response[i].full_name)   ;
    document.getElementById("demo").innerHTML += `    <img src="trash-bin.png" onclick="supp_a_friend('${search_response[i].id}');"></img>`;
    
    document.getElementById("demo").innerHTML += "</br>";
   

    }
}

async function supp_a_friend(id){

let xhr = new XMLHttpRequest();
let cookie_to_send = document.cookie.substring(4,200);
console.log(cookie_to_send);

let param = `id2=`+ id;
xhr.open("POST",url +`/supp_friend/?` + param, true);
xhr.setRequestHeader( "Authorization","Bearer " + cookie_to_send );
xhr.send(null)
await delay(300)
window.location.reload();


}
function suppr_account(){

    let xhr = new XMLHttpRequest();
    let cookie_to_send = document.cookie.substring(4,200);
    console.log(cookie_to_send);
    xhr.open("GET",url +`/suppr_account/`, true);
    xhr.setRequestHeader( "Authorization","Bearer " + cookie_to_send );
    xhr.send(null)
   
    }

function parseData(response) {
console.log("dans parse data")
search_response = JSON.parse(response)
console.log("search_response : ")
console.log(search_response)
document.getElementById("response").innerHTML = `</br>`;
for (var i = 0; i < search_response.length; i++) {
    document.getElementById("response").innerHTML += `<button onclick="func('${search_response[i].id}','${search_response[i].full_name}');">ajouter</button>`;
    document.getElementById("response").innerHTML += (search_response[i].full_name)
    document.getElementById("response").innerHTML += "</br>";
   
}
}
function getData() {
let xhr = new XMLHttpRequest();
let cookie_to_send = document.cookie.substring(4,200)
console.log(cookie_to_send)
xhr.open("GET",url +`/me`, true);
//xhr.setRequestHeader("Access-Control-Allow-Origin", "*");
xhr.setRequestHeader( "Authorization","Bearer " + cookie_to_send );
xhr.setRequestHeader( "Access-Control-Allow-Methods","*");
//xhr.setRequestHeader("Authorization", "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJmN2E5MDhiZi05M2NlLTRlNzctYWVhZS1kZDkyYmUyM2ZmYjQiLCJleHAiOjE2NDE0OTU3MTV9.pFUIrn2OLUDlYunuMUmJCxI3n-Lnd12PTAAXc4-rYzE");

console.log(document.cookie);


//   "authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJmN2E5MDhiZi05M2NlLTRlNzctYWVhZS1kZDkyYmUyM2ZmYjQiLCJleHAiOjE2NDE0OTU3MTV9.pFUIrn2OLUDlYunuMUmJCxI3n-Lnd12PTAAXc4-rYzE"
//document.cookie js prend les cookies du site
// xhr.onload = () => {
//     if (xhr.readyState === 4) {
//         if (xhr.status === 200) {
//             console.log(xhr.statusText);
//             parseData(xhr.responseText);
//         } else {
//             console.error(xhr.statusText);
//         }
//     }
// };
// xhr.onerror = () => {
//     console.error(xhr.statusText);
// };
xhr.send(null);
}


// var docFrag = document.createDocumentFragment();
// for (var i=0; i < 5 ; i++){
//      var elem = document.createElement('input');
//      elem.type = 'button';
//      elem.value = 'button';
//      docFrag.appendChild(elem);
// }

//     console.log(search_response.SearchUsers[0].id)
// document.getElementById("response").innerHTML += (search_response[1].id)

function delay(n){
return new Promise(function(resolve){
    setTimeout(resolve,n);
});
}

async function func(id,full_name)
{
let xhr = new XMLHttpRequest();
let cookie_to_send = document.cookie.substring(4,200);
console.log(cookie_to_send);
console.log(full_name);
let param = `friend_id_to_add=`+ id + `&friend_full_name=` + full_name;
xhr.open("POST",url +`/add_friend/?` + param, true);
xhr.setRequestHeader( "Authorization","Bearer " + cookie_to_send );
xhr.send(name)
await delay(300)
window.location.reload();


}

function getData_users() {
let xhr = new XMLHttpRequest();
let cookie_to_send = document.cookie.substring(4,200);
console.log(cookie_to_send);
xhr.open("GET",url +`/get_users`, false);
xhr.setRequestHeader( "Authorization","Bearer " + cookie_to_send );
xhr.setRequestHeader( "Access-Control-Allow-Methods","*");
xhr.send(null);
}




function search() {
const val = document.querySelector('input').value;
console.log(val);
let xhr = new XMLHttpRequest();
let cookie_to_send = document.cookie.substring(4,200);
console.log(cookie_to_send);
xhr.open("GET",url +`/get_users/` + val, false);
xhr.setRequestHeader( "Authorization","Bearer " + cookie_to_send );
xhr.setRequestHeader( "Access-Control-Allow-Methods","*");
 xhr.onload = () => {
    if (xhr.readyState === 4) {
        if (xhr.status === 200) {
            console.log(xhr.statusText);
            parseData(xhr.responseText);
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