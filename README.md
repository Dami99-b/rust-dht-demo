# Rust DHT Demo

A toy Distributed Hash Table (DHT) implementation in Rust.  

## ✨ Features
- Simulates a network of peers storing key-value pairs  
- Lookups can be performed from any peer in the network  
- Demonstrates decentralization and peer discovery basics  

## 📖 How It Works
1. Each peer has a unique ID and stores some key-value pairs.  
2. When a lookup is made, the peer routes the request to the responsible node.  
3. Data can be inserted and retrieved from the network without a central server.  

## 🚀 Future Extensions
- Add Kademlia-style routing (bucket-based peer storage)  
- Add replication for fault tolerance  
- Integrate libp2p for real networking  

## 🔗 Relevance
DHTs are the backbone of many decentralized systems:
- BitTorrent uses DHTs for peer discovery  
- Ethereum and IPFS rely on DHTs for distributed storage and networking  

This project is a **learning tool** and a **foundation** for more advanced P2P and blockchain infrastructure.
