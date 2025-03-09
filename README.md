# Consistent Core POC 

## Why Consistent Core
Some distributed system may have hundreds or thousands of nodes. Managing such a cluster means we need to keep track of which nodes are parts of which
shards, for example. 

Additionally, we need a mapping of keys to logical partitions because otherwise, every time nodes are added or removed, the entire data set would need to be moved, which is impractical in large-scale systems.

While the `gossip protocol` and consistent hashing enable scalability, critical data operations (such as membership metadata, global configuration, or leader election) require stronger consistency. We can achieve this by consitent core. Zookeeper and etcd are proper implementation of this. 




## Flow
```mermaid
sequenceDiagram
    participant J as NodeA
    participant N as NodeB    
    participant L as ConsistentCore
    

    J->>L: registerLease("Migo Group")
    L-->>J: registerLease("Migo Group", timeToLive=5s)
    
    Note over L: name: "Migo Group"<br/>value: NodeA<br/>timeToLive: 5s
    
    N->>L: watch("Migo Group")
    

    J --x L : Failed to send heartbeat
    
    
    L->>L: checkLease()
    L->>L: expire("Migo Group")
    L-->>N: Event("key_deleted", "Migo Group")
    
    N->>L: registerLease("Migo Group")
    L-->>L: registerLease("Migo Group", timeToLive=5s)
    
    Note over L: name: "Migo Group"<br/>value: NodeB<br/>timeToLive: 5s
```