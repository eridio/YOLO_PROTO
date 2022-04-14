https://aralroca.com/blog/first-steps-webassembly-rust


Let's compile Rust's code with:

> wasm-pack build --target web


run it :
 npx serve . --> localhost:3000

 rust signal-protocol lib:
 https://github.com/dingtype/libsignal-protocol-rust

jS: https://github.com/blockrockettech/signal-webapp

 https://github.com/Dione-Software/double-ratchet-2

 https://github.com/Dione-Software/x3dh-ke

 
 etapes echanges messages :
    - registration ID and store it
    - generate an identity key pair and store it 
    - one-time use prekey and a signed pre-key and store it 
    - Finally we store the bundle in the server


    etapes apres:
    - get a prekey bundle. from the server  (si alice veut communiquer avec bob elle recupere SON bundle (celui de bob))
    - calcul la MASTER KEY


    - envoie le premier message avec double ratchet 

    


a changer : 
   - 2 key init -> envois au serveur
   - alice_init_ratchet envoyer dans le premier message la clef public de bob (web rtc uniquement)
   - clean localstorage a la fin de la com