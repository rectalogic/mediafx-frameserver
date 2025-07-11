# MediaFX Frameserver

Implements "out of process" video effect plugins for [frei0r](https://dyne.org/software/frei0r/) -
enables video processing plugins to be written in Python, Node.js or Rust
running as external applications.

The frei0r plugins communicate with an external client process
using pipes and shared memory.
Source video frames are sent to the client which can apply visual effects
and return the modified frame.

## Usage

Download the frei0r plugins for your platform [![GitHub Release](https://img.shields.io/github/v/release/rectalogic/mediafx-frameserver)](https://github.com/rectalogic/mediafx-frameserver/releases/).

Install the client library for
Rust [![Crates.io Version](https://img.shields.io/crates/v/mediafx)](https://crates.io/crates/mediafx),
NodeJS [![NPM Version](https://img.shields.io/npm/v/%40mediafx%2Fclient)](https://www.npmjs.com/package/@mediafx/client),
or Python [![PyPI - Version](https://img.shields.io/pypi/v/mediafx)](https://pypi.org/project/mediafx/).

See example plugin clients for [Rust](frei0r/mediafx/examples), [NodeJS](clients/mediafx_node/examples) and [Python](clients/mediafx_py/python/examples).

See the [GitHub actions workflow](.github/workflows/frameserver-ci.yml) for examples of using the clients with FFmpeg and MLT.

## Diagram

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
