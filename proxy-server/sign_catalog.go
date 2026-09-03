package main

import (
	"crypto/ed25519"
	"crypto/x509"
	"encoding/hex"
	"encoding/pem"
	"log"
	"os"
)

var signingKey ed25519.PrivateKey
var publicKey ed25519.PublicKey

func loadSigningKey() {
	keyPath := os.Getenv("SIGNING_KEY_PATH")
	if keyPath == "" {
		log.Printf("SIGNING_KEY_PATH not set, signing disabled")
		return
	}
	keyData, err := os.ReadFile(keyPath)
	if err != nil {
		log.Printf("Failed to read signing key: %v", err)
		return
	}
	block, _ := pem.Decode(keyData)
	if block == nil {
		log.Printf("Failed to decode PEM block")
		return
	}
	priv, err := x509.ParsePKCS8PrivateKey(block.Bytes)
	if err != nil {
		log.Printf("Failed to parse PKCS#8 key: %v", err)
		return
	}
	key, ok := priv.(ed25519.PrivateKey)
	if !ok || len(key) != ed25519.PrivateKeySize {
		log.Printf("Key is not Ed25519 private key (len=%d)", len(key))
		return
	}
	signingKey = key
	publicKey = key.Public().(ed25519.PublicKey)
	log.Printf("Signing key loaded successfully from %s", keyPath)
}

func signData(data []byte) string {
	if len(signingKey) == 0 {
		return ""
	}
	sig := ed25519.Sign(signingKey, data)
	return hex.EncodeToString(sig)
}

func getPublicKeyHex() string {
	if len(publicKey) == 0 {
		return ""
	}
	return hex.EncodeToString(publicKey)
}
