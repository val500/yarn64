# yarn64 - Yet Another Rust N64 emulator

Very very early stage n64 emulator written in Rust to learn more about Rust and MIPS/computer architecture.

## Progress
### 7/30/20
- Currently working on the vr4300 CPU. Datasheet can be found here: http://datasheets.chipdb.org/NEC/Vr-Series/Vr43xx/U10504EJ7V0UMJ1.pdf
- vr4300 module contains several structs
  - The pipeline struct contains seperate types for each stage of the pipeline. 
  - Currently working on the "Executor" which will emulate the execution stage of the pipeline.
