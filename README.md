# Wineglass Documentation

## Overview

**Wineglass** is a modular, fault-tolerant, and server-focused compiled programming language designed to offer dynamic module loading and unloading, as well as concurrency with memory safety. Wineglass is ideal for environments where uptime and reliability are critical, supporting features like module hot-swapping, error handling, and a flexible IPC mechanism.

## System Features

### Module Management

Modules in Wineglass are precompiled binary files, loaded and unloaded via requests to the **Overseer**. The modules are independent of any specific module type, as long as they are located in the correct location, they can be used. Once a module (or "bottle") is loaded, it initializes itself. 

- **Dynamic Loading**: Modules are loaded by the Overseer through open requests. The loading process is managed by the Overseer.
- **Unloading**: Bottles can be unloaded via the `unload()` method, or completely removed from memory if there are no references left.
  
### Shatter Mechanism

A **shatter** occurs when a bottle/module decides it cannot safely operate anymore or is in a dangerous state. This is equivalent to a **BSOD** (Blue Screen of Death) in Windows or a **Kernel Panic** in Linux.

- Shatter states are represented by `u64` values, and they are critical for managing unrecoverable errors.
- The Overseer acts as a journal to log these states and manages requests for error handling.

### Error Handling and Recovery

Wineglass has several error states, denoted by `u64` values:

- **Warning**: Minor issues that do not affect functionality.
- **Error**: Issues that need to be addressed but do not immediately compromise functionality.
- **Critical**: Severe issues that may compromise system stability.
- **Shatter**: An unrecoverable state indicating a catastrophic failure.

Recovery from errors can be achieved by completely unloading and reloading the affected module. The Overseer ensures that all messages in the queue are processed before unloading the module unless a forced unload is requested via the `ovs.request(UNLOAD).force()` call.

### Concurrent Execution and IPC

Bottles (modules) can perform **Inter-Process Communication (IPC)** via shared memory and a transaction-based request/response method. 

- **Request-Response Model**: Bottle A can request Bottle B to execute an instruction. Bottle A locks until Bottle B completes the process, notifying Bottle A that the task is finished.
  - The locking mechanism can be adjusted with flags, allowing either **spin-waiting** or waiting for a direct notification.
- **Concurrency**: Wineglass ensures that no race conditions occur. A bottle will wait for another operation to finish if there is a potential for a race.
- **IPC Mechanism**: 
  - IPC operates via a memory pool or channels.
  - Messages are structured as data types (e.g., structs) and can be sent across bottles. 
  - IPC is not inherently cross-platform but uses abstractions to allow communication via different libraries, depending on the platform.

### Garbage Collection

Wineglass uses a **stack-based** garbage collection (GC) model. Each bottle deallocates itself when it's no longer in use, and there is no need for an explicit garbage collector.

### Overseer

The **Overseer** acts as the central controller of the Wineglass environment, functioning as:

- **Journal**: It logs the state of all bottles, including their error states (Warning, Error, Critical, Shatter).
- **Execution Manager**: It spawns, manages, and monitors bottles.
- **Error Recovery**: It handles critical errors (shatter states) by unloading and reloading modules.

- The Overseer can print logs to `stdout` as needed for visibility.
- It can force an abort in case of an error via `ovs.request(UNLOAD).force()`, except when certain flags prevent this action.

### Cross-Compilation

Wineglass supports **cross-compilation**, similar to how Rust handles cross-platform builds. It abstracts platform-specific details for IPC but does not inherently support cross-platform IPC.

### Bottles as Sandboxes

Each bottle operates in isolation like a **sandbox**. It is designed to prevent interfering with other bottles unless explicitly requested. This model allows for secure, concurrent execution of code without the risk of one bottle affecting the operation of another.

---

## Example Use Cases

### Dynamic Module Loading

```rust
// Example: Loading a module via the overseer
let module = load_module("path_to_module");

// The bottle initializes itself once loaded
let bottle = spawn_bottle(module);

// Perform actions within the bottle
execute_instruction(bottle, "some_instruction");

// Handle error states gracefully
if is_shattered(bottle) {
    overseer.handle_shatter(bottle);
}
