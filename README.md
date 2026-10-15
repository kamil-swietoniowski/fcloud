# Fcloud
---
## Personal Cloud 
A private system written in **Rust** for storing being able to access your files easily and safely

```mermaid
    graph LR
        A(App/Browser)
        B(Rust Backend)
        C[(Database)]
        D(Files on disk)
        A --> |Login & HTTPS|B
        B --> |Meta & Info| C
        B --> |Raw Files| D
```

---
## Philosophy
### Why?
* I was tired of having my data un organized
* I didnt want to share my data for free with corporation
* A practice, first big project
* I want to be able to query my informations fast

### Architecture
```mermaid 
    graph LR 
        A(Frondend)
        B(Backend)
        C(MediaStorage)
        A <--> B
        B <--> C
```

During this time **Frontend** will be written in typical web stack (HTML/CSS/JAVASCRIPT) backend in **Rust** and use **sqlite** for a database.

### Scheme
For first MVP
```mermaid
erDiagram
    Person {
    string id pk
    string name
    string last_name
    string phone_number
    string email_address
    string description
    }
    Place {
        string id pk
        string display_name
        string geo_coords
        string description
    }
    Note {
        string id pk
        string title
        string content
        date edited
    }
    Relation {
        string id pk
        string id_1 fk
        string id_2 fk
        string id_1_table
        string id_2_table
    }

    Person ||--o{ Relation: Relates
    Place ||--o{ Relation: Relates
    Note ||--o{ Relation: Relates

```

## How to run
Not yet