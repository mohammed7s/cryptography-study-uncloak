def vigenere(key, message):
    message = message.lower()
    message = message.replace(' ','')
    m = len(key)
    cipher_text = ''
    for i in range(len(message)):
        letter = message[i]
        k= ord(key[i % m])-97
        cipher_text = cipher_text + chr ((ord(letter) - 97 + k ) % 26 + 97)
    return cipher_text


def vigenere_decrypt(key, ciphertext):
    message = ''
    m = len(key) 
    for i in range(len(ciphertext)):
        letter = ciphertext[i]
        k = -1*(ord(key[i % m])-97)
        message = message + chr ((ord(letter) - 97 + k ) % 26 + 97)
    return message 


if __name__ == "__main__": 
  message = input("Enter the message: ")
  key = input("Enter the keyword: ")
  encrypt_text = vigenere(key,message) 
  print("Encrypted message:", encrypt_text) 
  print("Decrypted message:", vigenere_decrypt(key, encrypt_text)) 
