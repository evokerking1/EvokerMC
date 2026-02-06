# JNI Integration Guide

## Overview

EvokerMC now includes full JNI (Java Native Interface) integration for loading and running Java-based mods. This allows the engine to be compatible with existing Minecraft mods that use the Forge or Neoforge APIs.

## Architecture

The JNI integration consists of three main components:

### 1. JVM Manager (`evoker-modding/src/jvm.rs`)

The JVM Manager handles the lifecycle of the Java Virtual Machine:
- Initializing the JVM with appropriate settings
- Loading classes from JAR files
- Creating instances of Java classes
- Calling methods on Java objects
- Managing global references to prevent garbage collection

### 2. JAR Loader (`evoker-modding/src/jar.rs`)

The JAR Loader handles loading Java mods from JAR files:
- Extracting metadata from `mod.json` or `META-INF/MANIFEST.MF`
- Loading JAR files into the JVM classpath
- Instantiating the mod's main class
- Initializing the mod by calling its `init()` method

### 3. Java Mod (`JavaMod` struct)

Represents a loaded and initialized Java mod:
- Stores a global reference to the mod instance
- Provides methods to call Java methods from Rust
- Handles method invocation with JNI signatures

## Requirements

To use Java mods, the following must be installed:

1. **Java Development Kit (JDK)**
   - Version 8 or later
   - Download from: https://adoptium.net/ or https://www.oracle.com/java/technologies/downloads/

2. **Environment Setup**
   - `JAVA_HOME` environment variable must be set
   - The JVM native library must be in the library path:
     - Linux: `libjvm.so` (usually in `$JAVA_HOME/lib/server/`)
     - Windows: `jvm.dll` (usually in `%JAVA_HOME%\bin\server\`)
     - macOS: `libjvm.dylib` (usually in `$JAVA_HOME/lib/server/`)

### Setting Up on Linux

```bash
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
export LD_LIBRARY_PATH=$JAVA_HOME/lib/server:$LD_LIBRARY_PATH
```

### Setting Up on Windows

```cmd
set JAVA_HOME=C:\Program Files\Java\jdk-17
set PATH=%JAVA_HOME%\bin\server;%PATH%
```

### Setting Up on macOS

```bash
export JAVA_HOME=/Library/Java/JavaVirtualMachines/jdk-17.jdk/Contents/Home
export DYLD_LIBRARY_PATH=$JAVA_HOME/lib/server:$DYLD_LIBRARY_PATH
```

## Usage

### Creating a Java Mod

A Java mod must have one of the following:

1. A `mod.json` file in the root of the JAR:
```json
{
  "name": "Example Mod",
  "version": "1.0.0",
  "main": "com.example.ExampleMod",
  "dependencies": []
}
```

2. Or a `META-INF/MANIFEST.MF` with the following entries:
```
Implementation-Title: Example Mod
Implementation-Version: 1.0.0
Main-Class: com.example.ExampleMod
```

### Example Java Mod

```java
package com.example;

public class ExampleMod {
    // Default constructor (required)
    public ExampleMod() {
        System.out.println("ExampleMod constructor called");
    }
    
    // Called by EvokerMC when the mod is loaded
    public void init() {
        System.out.println("ExampleMod initialized!");
        // Register blocks, items, etc.
    }
    
    // Other methods can be called from Rust using JNI
    public void onTick() {
        // Called every game tick
    }
}
```

### Loading a Java Mod from Rust

```rust
use evoker_modding::{JarModLoader, JvmManager};
use std::path::Path;

// Initialize JVM (if available)
// Note: This requires a JDK to be installed
if let Ok(jvm) = JvmManager::init_from_existing(java_vm) {
    // JVM is now initialized
}

// Create JAR loader
let mut loader = JarModLoader::new();

// Load a JAR mod
let jar_path = Path::new("mods/example-mod.jar");
loader.load(jar_path).await?;

// Access loaded mods
for mod_meta in loader.loaded_mods() {
    println!("Loaded mod: {} v{}", mod_meta.name, mod_meta.version);
}

// Access Java mod instances
for java_mod in loader.java_mods() {
    println!("Java mod class: {}", java_mod.class_name());
    
    // Call a method on the mod
    java_mod.call_method("onTick", "()V", &[])?;
}
```

## JNI Signatures

When calling Java methods from Rust, you need to provide the correct JNI signature:

- `()V` - No parameters, void return
- `(I)V` - Int parameter, void return
- `()I` - No parameters, int return
- `(Ljava/lang/String;)V` - String parameter, void return
- `([I)V` - Int array parameter, void return

Examples:
```rust
// void method()
java_mod.call_method("method", "()V", &[])?;

// void method(int value)
java_mod.call_method("method", "(I)V", &[JValue::Int(42)])?;

// void method(String text)
let text = env.new_string("hello")?;
java_mod.call_method("method", "(Ljava/lang/String;)V", &[JValue::Object(&text)])?;
```

## API Compatibility

The JNI integration provides the foundation for Forge and Neoforge API compatibility:

- The `evoker-compat` crate provides compatibility layers
- Forge/Neoforge events are mapped to EvokerMC events
- Java mods can interact with the engine through the compatibility API

## Performance Considerations

- JNI calls have some overhead compared to native Rust code
- Global references are cached to avoid repeated lookups
- The JVM is initialized once and reused for all mods
- Consider batching JNI calls when possible

## Troubleshooting

### "Failed to create JVM"

- Ensure JDK is installed, not just JRE
- Check that `JAVA_HOME` is set correctly
- Verify the JVM native library is in the library path

### "Failed to find class"

- Check that the class name uses JNI format: `com/example/Class` not `com.example.Class`
- Ensure the JAR file has been loaded into the classpath
- Verify the class exists in the JAR file

### "Failed to call method"

- Check the method signature is correct
- Ensure the method exists and is public
- Verify the parameter types match the signature

## Future Improvements

- Custom ClassLoader for better mod isolation
- Hot-reloading of Java mods
- Bi-directional event system (Java ↔ Rust)
- Performance optimization of JNI calls
- Support for more complex method signatures
- Automatic signature generation

## References

- [JNI Specification](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/jniTOC.html)
- [jni-rs Documentation](https://docs.rs/jni/latest/jni/)
- [Minecraft Forge Documentation](https://docs.minecraftforge.net/)
