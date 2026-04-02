import base64, gzip, pyperclip

def xor(string: str, key: int) -> str:
 	return ("").join(chr(ord(char) ^ key) for char in string)

def decrypt_data(data: str) -> str:
    base64_decoded = base64.urlsafe_b64decode(xor(data, key=11).encode())
    decompressed = gzip.decompress(base64_decoded)
    return decompressed.decode()

if __name__ == "__main__":
    with open(r"C:\Users\lukki\AppData\Local\GeometryDash\CCLocalLevels.dat", encoding="utf8") as f:
        print(decrypt_data(f.read()))