use ckks_engine::*;
use log::info;



pub fn adv_string_operations(){
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // Set CKKS parameters: degree of polynomial (N = 2048) and prime modulus (q)
   let params = CkksParameters::new(2048, 1000000000000007);

   // Key generation
   let keygen = KeyGenerator::new();
   let (public_key, secret_key) = keygen.generate_keys();

   // Initialize CKKS encryptor and decryptor
   let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
   let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

   // Define some strings to encrypt and decrypt
   let string1 = "Hello, CKKS. This is ";
   let string2 = "Homomorphic Encryption of Strings";

   // Encrypt the strings
   info!("\n=== Encrypting Strings ===");
   let encrypted_string1 = encryptor.encrypt_string(string1);
   let encrypted_string2 = encryptor.encrypt_string(string2);

   let concatenate = encryptor.concatenate_encrypted_strings(&encrypted_string1,&encrypted_string2);

   let decrypted_string1 = decryptor.decrypt_string(&concatenate);

   info!("Decrypted Concatenated String : {:?}", decrypted_string1);

   let encrypted_substring = encryptor.extract_encrypted_substring(&encrypted_string1, ..3);

   let decrypted_string2 = decryptor.decrypt_string(&encrypted_substring);
   info!("Decrypted Sub String : {:?}", decrypted_string2);



}

fn main(){
    adv_string_operations();
}