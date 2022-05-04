import init, { hash, crypt_aes_gcm_siv, decrypt_aes_gcm_siv, crypt_aes_key, decrypt_aes_key, generateRandomString, new_CryptedMessage } from "../pkg/helloworld.js";
/*
async function chiffre(mdp) {
    await init();
    let clé = generateRandomString(32);
    console.log("clé " + clé);
    let messageDiv = document.getElementById("message");

    messageDiv.addEventListener("change", async function (event) {
        let message = messageDiv.value;
        console.log(message);
        let JSON = await send_crypted_message(message, mdp);
        console.log(JSON);
        extract_from_JSON(JSON, mdp);
        //chiffre_dechiffre(message);
    })

    let cipherKey = crypt_aes_key(clé, mdp);
    console.log("chiffre key aes: " + cipherKey);
    console.log("dechiffre key aes " + decrypt_aes_key(cipherKey, mdp));

    console.log("hash du mdp " + hash(mdp));
}

async function chiffre_dechiffre(messageAChiffrer) {
    await init()
    let messageChiffre = await crypt_aes_gcm_siv(messageAChiffrer);
    console.log(messageChiffre.key);
    let messageDechiffre = await decrypt_aes_gcm_siv(messageChiffre);
    console.log(messageDechiffre);
}
*/

//let mdpDiv = document.getElementById("mdp");

/*
mdpDiv.addEventListener("change", function (event) {
    let mdp = mdpDiv.value;
    console.log("mdp :" + mdp);
    chiffre(mdp);

})

*/

export async function send_crypted_message(message, password, conversationName, username) {
    await init();
    let cipher = crypt_aes_gcm_siv(message);
    let key = cipher.get_key();
    let crypted_key = crypt_aes_key(key, password);
    let JSON = {
        "username": username,//username
        "messages": cipher.get_text(),
        "key": crypted_key,
        "nonce": cipher.get_nonce(),
        "conversationName": conversationName,
        "date": new Date().toLocaleDateString()

    };
    /*
    const xhr = new XMLHttpRequest();
    xhr.open("POST", "url");
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(JSON);*/
    console.log(JSON);
    return JSON;
};

export function extract_from_JSON(JSON, password) {
    let cryptedMessage = new_CryptedMessage(JSON.messages, decrypt_aes_key(JSON.key, password), JSON.nonce);
    let plainText = decrypt_aes_gcm_siv(cryptedMessage);
    console.log("your messages : " + plainText);
    return plainText;
}