# MediaFX Frameserver

Implements "out of process" video effect plugins for [frei0r](https://dyne.org/software/frei0r/) -
enables video processing plugins to be written in Python, Node.js or Rust
running as external applications.

The frei0r plugins communicate with an external client process
using pipes and shared memory.
Source video frames are sent to the client which can apply visual effects
and return the modified frame.

```mermaid
graph TB
    subgraph "frei0r Host Application"
        A[MLT, FFmpeg etc.]
    end

    subgraph "frei0r Plugin"
        B[frei0r Frameserver Plugins]
        B1[Source Plugin]
        B2[Filter Plugin]
        B3[Mixer2 Plugin]
        B4[Mixer3 Plugin]
        B --> B1
        B --> B2
        B --> B3
        B --> B4
    end

    subgraph "Client Process"
        C[Video Processing Client]
        C1[Python Client]
        C2[Node.js Client]
        C3[Rust Client]
        C --> C1
        C --> C2
        C --> C3
    end

    subgraph "IPC Layer"
        D[Anonymous Pipes<br/>Control & Metadata]
        E[Shared Memory<br/>Video Frames]
    end

    A -->|f0r_update/f0r_update2| B
    B <-->|Control Messages| D
    B <-->|Video Frame Data| E
    D <--> C
    E <--> C
    B -->|Processed Frame| A

    style A fill:#ff0000
    style B fill:#aa0000
    style B1 fill:#aa0000
    style B2 fill:#aa0000
    style B3 fill:#aa0000
    style B4 fill:#aa0000
    style C fill:#0000ff
    style C1 fill:#0000ff
    style C2 fill:#0000ff
    style C3 fill:#0000ff
    style D fill:#00aa00
    style E fill:#00aa00
```
