# MediaFX Frameserver

Implements "out of process" video effect plugins for [frei0r](https://dyne.org/software/frei0r/)

The frei0r plugins communicate with an external client process
using pipes and shared memory.
Video frames are sent to the client which can apply visual effects
and return the modified frame.
