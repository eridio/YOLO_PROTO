pub mod lib_3XDH;
use lib_3XDH::*;
pub mod src_2R;
use src_2R::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use serde::{Serialize,Deserialize};
use serde::ser::{SerializeTupleStruct,Serializer};
use serde_json::*;
use p256::{
    ecdsa::{ Signature}};
use p256::{PublicKey, SecretKey};

use sha2::{Sha256, Digest};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`
use aes_gcm_siv::aead::{Aead, NewAead};
use rand::{distributions::Alphanumeric, Rng};
use pbkdf2::{
    password_hash::{PasswordHasher},
    Pbkdf2
};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Bundle {
    identity_key : IdentityKey,
    signed_pre_key: SignedPreKey,
    signature : Signature,
    one_time_pre_key : OneTimePreKey,
    ephemeral_key : EphemeralKey,
}

#[wasm_bindgen]
#[derive(Debug,Deserialize,Serialize)]
pub struct Messages {
    id_issuer : String,
    id_room : String,
    date : String,
    encrypted : String,
    header : String,
    nonce : String,
}
#[wasm_bindgen]
#[derive(Debug,Deserialize,Serialize)]
pub struct MessagesTest {
    encrypted : String,
    header : String,
    nonce : String,
}

#[wasm_bindgen]
#[derive(Debug,Deserialize,Serialize)]
pub struct IdentityStringify {
    name_ : String,
    ephemeral_key : String,
    identity_key : String,
    signed_pre_key: String,
    signature_ : String,
    one_time_pre_key : String
}



#[derive(Debug,Deserialize,Serialize)]
pub struct NameOf {
pub name_ : String,

}
#[wasm_bindgen]
#[derive(Debug,Deserialize,Serialize)]
pub struct Identity {
name_ : String,
#[serde(with = "serde_bytes")]
identity_key : Vec<u8>,
#[serde(with = "serde_bytes")]
signed_pre_key: Vec<u8>,
#[serde(with = "serde_bytes")]
signature : Vec<u8>,
#[serde(with = "serde_bytes")]
one_time_pre_key : Vec<u8>,
#[serde(with = "serde_bytes")]
ephemeral_key : Vec<u8>,
}
#[wasm_bindgen]
#[derive(Debug,Deserialize,Serialize)]
pub struct InitAlice {
    bundle_server : Identity,
    bundle_keep : IdentityStringify
}


#[derive(Debug,Serialize,Deserialize)]
pub struct RatchetStringify {
    pub dhs_private: Vec<u8>,
    pub dhs_public : String,
    pub dhr: String,
    pub rk: [u8; 32],
    pub ckr: [u8; 32],
    pub cks: [u8; 32],
    pub ns: String,
    pub nr: String,
    pub pn: String,
    pub mkskipped:String
}

#[wasm_bindgen]
#[derive(Debug,Serialize,Deserialize)]
pub struct RatchetAndPubKey {
    ratchet : String,
    pub_key : String,
} 

#[wasm_bindgen]
pub fn hash(mut password:String)->String{
      // create a Sha256 object
      const SALT:&str="a31158c6-9cb0-11ec-b909-0242ac120002";

      let mut hasher = Sha256::new();
      
      password.push_str(SALT);
      // write input message
      hasher.update(password);
  
      // read hash digest and consume hasher
      let result = hasher.finalize();
      
      return format!("{:x}", result);
}

#[wasm_bindgen]
pub struct CryptedMessage{
    cipherText: Vec<u8>,
    key: String,
    nonce:String, 
}
#[wasm_bindgen]
#[derive(Serialize,Deserialize,Debug)]
pub struct JSONToSend{
    username:String,
    cipherText: Vec<u8>,
    key: Vec<u8>,
    nonce:String, 
    conversationName:String,
    date:String
}


#[wasm_bindgen]
pub fn new_JSONToSend(username:String,cipherText: Vec<u8>,key: Vec<u8>,nonce:String,conversationName:String,date:String)->String{
    let new_json=JSONToSend{
        username:username,
        cipherText:cipherText,
        key:key,
        nonce:nonce,
        conversationName:conversationName,
        date:date,
    };
    return serde_json::to_string(&new_json).unwrap();
}
#[wasm_bindgen]
pub fn new_CryptedMessage(cipherText: Vec<u8>,key: String,nonce:String)->CryptedMessage{
    let crypted_message= CryptedMessage{
        cipherText:cipherText,
        key:key,
        nonce:nonce
    };
    return crypted_message;
}

#[wasm_bindgen]
impl CryptedMessage{
    pub fn get_text(&self)->Vec<u8>{
        self.cipherText.clone()
    }
    pub fn get_key(&self)->String{
        self.key.clone()
    }
    pub fn get_nonce(&self)->String{
        self.nonce.clone()
    }
}

#[wasm_bindgen]
pub fn pbkdf2_derivation(password : String) -> String{
    let password_to_hash = password.as_bytes();
    let salt = "hoBKFfPpeuP5OEO1UxaC42";
    Pbkdf2.hash_password_simple(password_to_hash, salt).unwrap().to_string()
}
#[wasm_bindgen]
pub fn truncate(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => s.to_string(),
        Some((idx, _)) => (&s[..idx]).to_string(),
    }
}
#[wasm_bindgen]
pub fn generateRandomString(size:usize)->String{
    let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(size)
    .map(char::from)
    .collect();
    return s;
}

#[wasm_bindgen]
pub fn crypt_aes_gcm_siv(message:String)->CryptedMessage{
   

    let rand_key=format!("{}",generateRandomString(32));
    let aeskey = Key::from_slice(rand_key.as_bytes());    
    let cipher = Aes256GcmSiv::new(aeskey);

    let rand_string=format!("{}",generateRandomString(12));
    let nonce = Nonce::from_slice(rand_string.as_bytes()); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(nonce, message.as_ref())
        .expect("encryption failure!");  // NOTE: handle this error to avoid panics!
        println!("encrypted text:{}",format!("{:?}",ciphertext));

    
    let crypted_message = CryptedMessage{
        cipherText: ciphertext,
        //key:reduced_key.to_string(),
        key:rand_key.to_string(),
        nonce: rand_string,
    } ;
    
    return crypted_message;
}
#[wasm_bindgen]
pub fn decrypt_aes_gcm_siv(crypted_message:CryptedMessage)->String{
    let key = Key::from_slice(crypted_message.key.as_bytes());
    let cipher = Aes256GcmSiv::new(key);
    let nonce=Nonce::from_slice(crypted_message.nonce.as_bytes());
    let plaintext = cipher.decrypt(&nonce, crypted_message.cipherText.as_ref())
        .expect("decryption failure!");  // NOTE: handle this error to avoid panics!

    let message = String::from_utf8_lossy(&plaintext);
    println!("Clear text:{}",message);
    return message.to_string();
}


#[wasm_bindgen]
pub fn crypt_aes_key(key:String,password : String)->Vec<u8>{
    let secret_key=String::from(pbkdf2_derivation(password));
    let  split=secret_key.split("$");
    let vec: Vec<&str> = split.collect();
    //println!("{}",secret_key);
    let reduced_key=truncate(vec[4], 32);
    let key_aes = Key::from_slice(reduced_key.as_bytes());
    let cipher = Aes256GcmSiv::new(key_aes);

  //  let mut block=GenericArray::from_slice(key.as_bytes()).clone();
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
   // let mut block2 = block.clone();

    
        let crypted_key = cipher.encrypt(nonce, key.as_ref())
        .expect("encryption failure!");  // NOTE: handle this error to avoid panics!
        // println!("encrypted text:{}",format!("{:?}",ciphertext));
        return crypted_key;

    
    //let new_block=block;
    
}

#[wasm_bindgen]
pub fn decrypt_aes_key(crypted_key:Vec<u8>,password:String)->String{
    let secret_key=String::from(pbkdf2_derivation(password));
    let  split=secret_key.split("$");
    let vec: Vec<&str> = split.collect();
    //println!("{}",secret_key);
    let reduced_key=truncate(vec[4], 32);
    let key_aes = Key::from_slice(reduced_key.as_bytes());
    let cipher = Aes256GcmSiv::new(key_aes);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let key = cipher.decrypt(nonce, crypted_key.as_ref())
    .expect("decription failure!");  // NOTE: handle this error to avoid panics!
    return String::from_utf8_lossy(&key).to_string();

}


pub fn parse_string_from_ratchet(r : Ratchet) -> String{

    let dhs_private = r.dhs.ex_private_key_bytes(); //vec
    let dhs_public = r.dhs.public_key.to_jwk_string();//string
        
    let mut dhr_test = r.dhr.clone();
    let mut dhr_to_return = String::from("");
    if (dhr_test).is_some(){
        dhr_to_return = dhr_test.ok_or("").unwrap().to_jwk_string();
    }else{
        dhr_to_return = "".to_string();
    }

    println!("{:?}" , dhr_test);
    
    let rk_test = r.rk;
    
    let mut ckr_test = r.ckr;
    let mut ckr_to_return = [0;32];
    if (ckr_test).is_some(){
        ckr_to_return = ckr_test.ok_or("").unwrap();
    }else
    {
        ckr_to_return = [0;32];
    }
    let mut cks_test = r.cks;
    let mut cks_to_return = [0;32];
    if (cks_test).is_some(){
        cks_to_return = cks_test.ok_or("").unwrap();
    }else
    {
        cks_to_return = [0;32];
    }
    let mut ns_test = r.ns;
    let mut ns_test = ns_test.to_string();
    
    let mut nr_test = r.nr;
    let mut nr_test = nr_test.to_string();

    let mut pn_test = r.pn;
    let mut pn_test = pn_test.to_string();

    let mkskipped_test = r.mkskipped.clone();
    
    let mut mkskipped_test =  format!("{:?}", mkskipped_test); //ce qu'on envois

    let ratchet_to_send = RatchetStringify {
        dhs_private: dhs_private,
        dhs_public : dhs_public,
        dhr:  String::from(dhr_to_return),
        rk : rk_test,
        ckr: ckr_to_return,
        cks: cks_to_return,
        ns: ns_test,
        nr: nr_test,
        pn: pn_test,
        mkskipped:mkskipped_test,
        };
    serde_json::to_string(&ratchet_to_send).unwrap()
}


pub fn parse_ratchet_from_string(s:String) -> Ratchet{

    let ratchet : RatchetStringify = serde_json::from_str(&s).unwrap();

    let dhs = DhKeyPair::from_bytes(ratchet.dhs_private,ratchet.dhs_public);//good form


    let mut dhr = ratchet.dhr;
    let mut dhr_to_return;
    if dhr == "".to_string() {
        dhr_to_return = None;
    }
    else{
    dhr_to_return=  Some(PublicKey::from_jwk_str(&dhr).unwrap());//good form
    
    }
        
    let rk_to_return = ratchet.rk;
    

    
    let mut ckr_test = ratchet.ckr;
    let mut ckr_to_return;  
    if (ckr_test) == [0;32] {
        ckr_to_return = None
    }else{
        ckr_to_return = Some(ckr_test)
    }
    
    let mut cks_test = ratchet.cks;
    let mut cks_to_return;  
    if (cks_test) == [0;32] {
        cks_to_return = None
    }else{
        cks_to_return = Some(cks_test)
    }

    
    let ns_test = ratchet.ns;
    let ns_to_return = ns_test.parse::<usize>().unwrap();
    
    let nr_test = ratchet.nr;
    let nr_to_return = nr_test.parse::<usize>().unwrap();
    
    let pn_test = ratchet.pn;
    let pn_to_return = pn_test.parse::<usize>().unwrap();
    
    


    let mkskipped_test = ratchet.mkskipped.clone();
    
    let mut a = mkskipped_test; //ce qu'on envois
    let mut a : String = a.replace("{","");
    let mut a = a.replace("}","");
    let mut a = a.replace("(","");
    let mut counter = 0;
    let mut index : usize = 0;
    let mut index2 : usize = 0;
    let mut range =  a.matches(":").count();
    let mut vec_u8 : Vec<Vec<u8>> = Vec::new();
    let mut vec_usize :Vec<usize> = Vec::new();

    for i in a.chars().enumerate(){
        if i.1 == '[' || i.1 == ']'{
        counter+=1;
        
            if counter == 2 {
                index = i.0;

                //save vec u8
                let slice = &a[index2.clone()+1..index.clone()+1];  
                
                vec_u8.push(parse_bundle_arguments(String::from(slice)));
                
                //save usize
                let slice2 = &a[index.clone()+3..index.clone()+4];
                
                vec_usize.push( slice2.parse::<usize>().unwrap());
                
            }
            if counter == 4 {
                index2 = i.0;
                
                let slice3 = &a[index.clone()+7..index2.clone()+1];
                vec_u8.push( parse_bundle_arguments(String::from(slice3)));
                
                counter=0;
            }
        }
    
    }
    let mut hash_map_to_return :HashMap<(Vec<u8>, usize), [u8; 32]>=  HashMap::new();

    let mut ind = 0;
    
    for j in 0..vec_usize.len(){
        let c = vec_u8[ind+1].clone();
        let vec_to_push : [u8; 32] = (c).try_into().unwrap();
        hash_map_to_return.insert((vec_u8[ind].clone(),vec_usize[j]), vec_to_push);
        ind+=2;
    }

    let ratchet_to_return = Ratchet {
        dhs : dhs,
        dhr : dhr_to_return,
        rk : rk_to_return,
        ckr : ckr_to_return,
        cks : cks_to_return,
        ns : ns_to_return,
        nr : nr_to_return,
        pn : pn_to_return,
        mkskipped : hash_map_to_return 
    };
    ratchet_to_return

}


#[derive(Debug,Serialize, Deserialize)]
pub struct HeaderStringify {
    public_key : String,
    pn : String, //usize
    n : String, //usize
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MessageBundle {
    header : HeaderStringify,
    encrypted : Vec<u8>,
    nonce : [u8;12],
}
//POUR LE DECHIFFREMENT DE MESS
pub fn parse_header_from_string(s : String) -> (Header, Vec<u8>, [u8;12]) {
    let message_bundle : MessageBundle= serde_json::from_str(&s).unwrap();

    let public_key = PublicKey::from_jwk_str(&message_bundle.header.public_key).unwrap();
    let pn = message_bundle.header.pn.parse::<usize>().unwrap();
    let n = message_bundle.header.n.parse::<usize>().unwrap();
    let header_to_return = Header {
        public_key : public_key,
        pn : pn,
        n : n
    };
    let encrypted_to_return = message_bundle.encrypted;
    let nonce_to_return = message_bundle.nonce;

    (header_to_return,encrypted_to_return,nonce_to_return)


}
//POUR l'ENVOIS DE MESS
pub fn parse_string_from_header(h : Header, encrypted : Vec<u8> , nonce : [u8;12]) -> String{
    let public_key = h.public_key.to_jwk_string();
    let pn = h.pn.to_string();
    let n = h.n.to_string();
    let header_stringify = HeaderStringify{
        public_key : public_key,
        pn : pn,
        n : n
    };

    let message_bundle_to_return = MessageBundle {
        header : header_stringify,
        encrypted : encrypted,
        nonce : nonce
    };

    serde_json::to_string(&message_bundle_to_return).unwrap()
}


pub fn parse_bundle_arguments(s: String) -> Vec<u8> {
    //let mut buf = String::from("[0, 0, 0, 0, 0, 178, 0, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 70, 107, 119, 69, 119, 89, 72, 75, 111, 90, 73, 122, 106, 48, 67, 65, 81, 89, 73, 75, 111, 90, 73, 122, 106, 48, 68, 65, 81, 99, 68, 81, 103, 65, 69, 51, 110, 105, 67, 104, 85, 102, 57, 47, 53, 85, 116, 104, 118, 83, 105, 52, 68, 119, 47, 72, 48, 66, 113, 83, 86, 105, 103, 10, 56, 97, 122, 113, 77, 111, 113, 75, 76, 114, 122, 53, 116, 102, 55, 81, 101, 79, 114, 111, 113, 105, 74, 118, 83, 86, 52, 90, 118, 117, 78, 108, 90, 76, 110, 119, 83, 106, 85, 118, 79, 119, 122, 122, 49, 55, 72, 116, 99, 113, 75, 68, 104, 48, 99, 88, 56, 65, 61, 61, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10]");
    let fst_caract = &s[0..1];
   
    let mut s = s.clone();
    if fst_caract == ","{
    
        s = String::from(&s[2..]);
    }
  //  println!("c'est egal {}" , s);
    let mut s = s.replace("[", "");
    let mut s = s.replace("]", "");
    let t = s.replace(" ", ""); //enlever les spaces
    let a: Vec<String> = t.split(",").map(str::to_string).collect();
    //let int = s.parse::<u8>().unwrap();
    //println!("{:?}",a);
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..a.len() {
        vec.push(a[i].parse::<u8>().unwrap())
    }
    // println!("{:?}", vec);
    vec
}
//############################################### end of formating ####################

//------------------------------------------alice------------------
#[wasm_bindgen]
pub fn key_init_alice(caller : String) -> String{
    //1
    use crate::lib_3XDH::*;
    let ika = IdentityKey::default();
    let ikas = ika.strip();
    //2
    let spka = SignedPreKey::default();
    //3
    let eka = EphemeralKey::default();
    let ekas = eka.strip();
    //4
    let otpka = OneTimePreKey::default();
    let otpkas = otpka.strip();
    //bundle a envoyer au server
    let bundle_alice_to_keep = IdentityStringify {
        name_ : String::from(caller.clone()),
        ephemeral_key :format!("{:?}",eka.to_bytes()) ,
        identity_key: format!("{:?}",ika.to_bytes().clone()),
        signed_pre_key : format!("{:?}",spka.to_bytes().clone()),
        signature_ : format!("{:?}",(ika.sign(&spka.strip().pk_to_bytes()).as_ref()).to_vec()),
        one_time_pre_key: format!("{:?}",otpka.to_bytes())
    };

    let bundle_server = Identity {
        name_ : String::from(caller),

        identity_key : ika.clone().strip().to_bytes(),
        signed_pre_key: spka.strip().to_bytes(),
        signature : (ika.sign(&spka.strip().pk_to_bytes()).as_ref()).to_vec(),
        one_time_pre_key : otpkas.to_bytes(),
        ephemeral_key : ekas.to_bytes(),

    };
    //update la table post bundle 
    let return_bundle = InitAlice {
        bundle_server: bundle_server,
        bundle_keep: bundle_alice_to_keep
    };
    //pour l'instant stockage sur le nav
    serde_json::to_string(&return_bundle).unwrap()
}


#[wasm_bindgen]
pub fn calculate_master_key_alice(bundle_bob_server : String, bundle_alice_local : String ) -> String{
    use crate::lib_3XDH::*;

    let bundle_alice : IdentityStringify = serde_json::from_str(&bundle_alice_local).unwrap();
    let bundle_bob : Identity = serde_json::from_str(&bundle_bob_server).unwrap() ;

    //alice serialization
    let ika_to_vec = parse_bundle_arguments(bundle_alice.identity_key);
    let eka_to_vec = parse_bundle_arguments(bundle_alice.ephemeral_key);

    let ika = IdentityKey::from_bytes(&ika_to_vec).unwrap();
    let eka = EphemeralKey::from_bytes(&eka_to_vec).unwrap();

    //bob serialization
    let signature_parsed: &[u8] = &bundle_bob.signature;
    
    let signature = Signature::try_from(signature_parsed).unwrap();
    let spkbs = SignedPreKey::from_bytes(&bundle_bob.signed_pre_key).unwrap(); //deja strip dans le bundle
    let ikbs = IdentityKey::from_bytes(&bundle_bob.identity_key).unwrap();
    let opkbs = OneTimePreKey::from_bytes(&bundle_bob.one_time_pre_key).unwrap();
    //master key

    let cka = x3dh_a(&signature, &ika , &spkbs, &eka, &ikbs, &opkbs).unwrap(); //alice = bob , alice, bob ,alice, bob, bob
    //signature bob                 ->    server
    //identity_key alice            ->    local
    //signed_pre_key bob STRIP      ->    server
    //ephemeral alice               ->    local
    //identity_key BOB STRIP        ->    server
    //one time pre key bob STRIP    ->    server
    println!("{:?}", cka);
    serde_json::to_string(&cka).unwrap()
}
use src_2R::ratchet::*;
        use src_2R::header::Header;
        use hashbrown::HashMap;
        use src_2R::dh::DhKeyPair;
        
#[wasm_bindgen]
pub fn alice_init_ratchet(sk : String, bob_pub_key : String) -> String{

    let sk = parse_bundle_arguments(sk);
    let sk = sk.try_into().unwrap();
    let bob_pub_key = PublicKey::from_jwk_str(&bob_pub_key).unwrap();
    let mut alice_ratchet = Ratchet::init_alice(sk, bob_pub_key);
    let alice_ratchet_to_return = parse_string_from_ratchet(alice_ratchet);
    serde_json::to_string(&alice_ratchet_to_return).unwrap()
}
#[derive(Debug,Serialize,Deserialize)]
pub struct RatchetAndMessageBundle {
    ratchet : String,
    message_bundle : String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct RatchetAndDecryptedMessage {
    ratchet : String,
    decrypted : String
}

#[wasm_bindgen]
pub fn send(ratchet :String, plaintext: String) -> String{
    let mut alice_ratchet = parse_ratchet_from_string(ratchet);
    let ad = b"associated data";
    let data = plaintext.as_bytes().to_vec();
    let (header, encrypted, nonce) = alice_ratchet.ratchet_encrypt(&data, ad);

    let message_bundle = parse_string_from_header(header,encrypted,nonce);
    let ratchet_to_return = parse_string_from_ratchet(alice_ratchet);
    let ratchet_and_message_bundle_to_return = RatchetAndMessageBundle {
        ratchet: ratchet_to_return,
        message_bundle : message_bundle
    };
    serde_json::to_string(&ratchet_and_message_bundle_to_return).unwrap()
}
//---------------------------------------bob------------------
#[wasm_bindgen]
pub fn decrypt(ratchet : String, message_bundle : String) -> String{

    let mut bob_ratchet = parse_ratchet_from_string(ratchet);
    let ad = b"associated data";
    let (header, encrypted, nonce) = parse_header_from_string(message_bundle);

    let decrypted = bob_ratchet.ratchet_decrypt(&header, &encrypted, &nonce, ad);
    let ratchet_to_return = parse_string_from_ratchet(bob_ratchet);
    let ratchet_and_decrypted_message_to_return = RatchetAndDecryptedMessage{
        ratchet : ratchet_to_return,
        decrypted : String::from_utf8(decrypted).unwrap()
    };

    serde_json::to_string(&ratchet_and_decrypted_message_to_return).unwrap()
}


#[wasm_bindgen]
pub fn bob_init_ratchet(sk :String) -> String {

    let sk = parse_bundle_arguments(sk);
    let sk = sk.try_into().unwrap();
    let (mut bob_ratchet, public_key) = Ratchet::init_bob(sk);
    let ratchet_to_return = parse_string_from_ratchet(bob_ratchet);
    let pub_key_to_return = public_key.to_jwk_string();
    let return_bob_init = RatchetAndPubKey {
        ratchet : ratchet_to_return,
        pub_key: pub_key_to_return
    };
    serde_json::to_string(&return_bob_init).unwrap()
}


#[wasm_bindgen]
pub fn key_init_bob(callee : String)->String {
    use crate::lib_3XDH::*;
    let ikb = IdentityKey::default();
    let ikbs = ikb.strip();
    

    let spkb = SignedPreKey::default();
    let spkbs = spkb.strip();
    
    let ekb = EphemeralKey::default();
    let ekbs = ekb.strip();
    
    let opkb = OneTimePreKey::default();
    let opkbs = opkb.strip();
    
    let signature = ikb.sign(&spkbs.pk_to_bytes());
    
    let bundle_bob_to_keep = IdentityStringify {
        name_ : String::from(callee.clone()),
        ephemeral_key : format!("{:?}", ekb.to_bytes()),
        identity_key: format!("{:?}",ikb.to_bytes().clone()),
        signed_pre_key : format!("{:?}",spkb.to_bytes().clone()),
        signature_ : format!("{:?}",ikb.sign(&spkb.strip().pk_to_bytes())),
        one_time_pre_key: format!("{:?}",opkb.to_bytes())
    };

    let bundle_server = Identity {
        name_ : String::from(callee),
        identity_key : ikb.clone().strip().to_bytes(),
        signed_pre_key: spkb.strip().to_bytes(),
        signature : (ikb.sign(&spkb.strip().pk_to_bytes()).as_ref()).to_vec(),
        one_time_pre_key : opkbs.to_bytes(),
        ephemeral_key : opkbs.to_bytes()

    };
    //update la table post bundle 
    let return_bundle = InitAlice {
        bundle_server: bundle_server,
        bundle_keep: bundle_bob_to_keep
    };
    //pour l'instant stockage sur le nav
    serde_json::to_string(&return_bundle).unwrap()
}

#[wasm_bindgen]
pub fn calculate_master_key_bob(bundle_bob_local : String, bundle_alice_server : String ) -> String{

    use crate::lib_3XDH::*;
    let bundle_bob : IdentityStringify = serde_json::from_str(&bundle_bob_local).unwrap();
    let bundle_alice : Identity = serde_json::from_str(&bundle_alice_server).unwrap() ;

    

    //alice parsing
    let ikas = IdentityKey::from_bytes(&bundle_alice.identity_key).unwrap();
    let ekas = EphemeralKey::from_bytes(&bundle_alice.ephemeral_key).unwrap();

    //bob parsing
    let spbk_to_vec = parse_bundle_arguments(bundle_bob.signed_pre_key);
    let ikb_to_vec = parse_bundle_arguments(bundle_bob.identity_key);
    let opkb_to_vec = parse_bundle_arguments(bundle_bob.one_time_pre_key);

    let spkb = SignedPreKey::from_bytes(&spbk_to_vec).unwrap(); 
    let ikb = IdentityKey::from_bytes(&ikb_to_vec).unwrap();
    let opkb = OneTimePreKey::from_bytes(&opkb_to_vec).unwrap();
    

    let ckb = x3dh_b(&ikas, &spkb, &ekas, &ikb, &opkb);//bob =   alice, bob, alice, bob, bob
    //signature bob                 ->    server
    //identity_key alice            ->    local
    //signed_pre_key bob STRIP      ->    server
    //ephemeral alice               ->    local
    //identity_key BOB STRIP        ->    server
    //one time pre key bob STRIP    ->    server
    println!("{:?}", ckb);
    serde_json::to_string(&ckb).unwrap()
}