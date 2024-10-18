# Bottle States  

Bottles use **u32** values to represent their states.  

The `xx` in the hex codes represents **any valid value** within that range.

---

## Standard States  
These states represent the active lifecycle of a bottle (thread).  

- **`0x00000000`: Completed**  
  The bottle finished executing and is now idle.  

- **`0x00000001`: Racked**  
  The bottle has loaded and is in a static state, waiting to run.  

- **`0x00000002`: Executing**  
  The bottle is actively parsing and running code.  

---

## Paused States (`0x00001xxx`)  
These states indicate that the bottle is waiting and cannot proceed without specific input or conditions.  

- **`0x00010xx`: Sleeping (Idle)**  
  The bottle is operational but temporarily inactive, with no task scheduled.  

- **`0x00011xx`: Waiting**  
  The bottle is waiting for input, an event, or another task to resume.  

- **`0x00012xx`: Blocked**  
  The bottle is waiting for external resources (e.g., I/O, network).  

- **`0x00013xx`: Paused**  
  The bottle is paused temporarily by an interrupt.  

- **`0x00015xx`: Hard Paused**  
  A critical pause enforced by the system (e.g., an override).  

---

## Defective States (`0x1xxxxxxx`)  
These states indicate that the bottle encountered a failure that prevents it from continuing until fixed externally.  

- **`0x1000xxxx`: General Failure**  
  The bottle stopped due to an unknown issue.  

- **`0x110xxxxx`: Restart**  
  The bottle is restarting due to a critical error.  

---

## Errored States (`0x2xxxxxxx`)  
These represent the bottle failing a task and waiting to clean up.  

- **`0x20000xx`: General Error**  
  The bottle is recovering from an error and performing cleanup.  

---

## Critical States (`0x8xxxxxxx`)  
The bottle cannot run any further and requires a complete reset or shutdown.  

- **`0x80000000`: Unknown**  
  The bottle failed due to an unknown reason.  

- **`0x800000xx`: Killed**  
  The bottle was terminated by another process or itself.  

- **`0x800001xx`: Signal**  
  The bottle is handling a critical signal (e.g., SIGTERM).  

---

## Cascaded States (`0x9xxxxxxx`)
The bottle is errored, likely from another process causing an unrecoverable error raised by another bottle.

- **`0x90000

---

## Shattered States (`0xFxxxxxxx`)  
The bottle cannot operate whatsoever and is in an unrecoverable state.  

- **`0xF0xxxxxx`: Disposed**  
  The bottle has been handled by another thread and is ready for garbage collection.  

- **`0xF1xxxxxx`: Unhandled**  
  The bottle is halted without appropriate handling.  

- **`0xF2xxxxxx`: Empty**  
  The bottle shattered before loading anything, likely due to a preprocessing error.  

- **`0xFFxxxxxx`: Debug**  
  The bottle is shattered but waiting for debug. This occurs only if `!this [dbg]` is specified.  

---

## Fatal Errors (`u64`)  
Fatal errors use the full 64 bits, with the upper 32 bits containing **location or thread-specific data**. These errors indicate the halt of all threads, usually signaling a crash.  

---

## Example Hex Usage  
- **`0x0001002`**: The bottle is in idle sleep.  
- **`0x0001125`**: The bottle is waiting for an external signal.  
- **`0x0001301`**: The bottle is in a critical pause due to a system interrupt.  
