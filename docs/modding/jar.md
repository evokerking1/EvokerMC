# JAR Mods

EvokerMC supports Java-based mods packaged as `.jar` files. JAR mods are loaded via JNI (Java Native Interface), which means the Java Virtual Machine runs inside the same process as the engine.

## Requirements

- JDK 8 or later must be installed.
- The `JAVA_HOME` environment variable must point to the JDK root.
- The JVM native library must be on the library path (see [Installation](/installation)).

## Creating a JAR Mod

### 1. Write the Java Class

```java
package com.example;

public class MyMod {
    public MyMod() {
        System.out.println("MyMod constructor called");
    }

    /** Called by EvokerMC when the mod is initialized. */
    public void init() {
        System.out.println("MyMod initialized!");
    }

    /** Called every game tick. */
    public void onTick() {
        // per-tick logic
    }
}
```

The class **must** have a public no-argument constructor. EvokerMC calls `init()` after instantiation.

### 2. Create `mod.json`

Place this file at the **root** of the JAR (not inside a sub-directory):

```json
{
  "id":          "my-mod",
  "name":        "My Mod",
  "version":     "1.0.0",
  "description": "An example JAR mod",
  "authors":     ["YourName"],
  "dependencies": [],
  "main":        "com.example.MyMod"
}
```

Alternatively you can use `META-INF/MANIFEST.MF`:

```
Implementation-Title: My Mod
Implementation-Version: 1.0.0
Main-Class: com.example.MyMod
```

### 3. Package as a JAR

```bash
javac -d out src/com/example/MyMod.java
cp mod.json out/
jar cf my-mod-1.0.0.jar -C out .
```

### 4. Install the Mod

```bash
cp my-mod-1.0.0.jar mods/
```

Start EvokerMC. The mod loader discovers and loads the JAR automatically.

## Loading Mods from Rust

For engine contributors and advanced use cases:

```rust
use evoker_modding::{JarModLoader, JvmManager};
use std::path::Path;

// Initialize the loader
let mut loader = JarModLoader::new();

// Load a single JAR
loader.load(Path::new("mods/my-mod-1.0.0.jar")).await?;

// Inspect loaded mods
for meta in loader.loaded_mods() {
    println!("Loaded: {} v{}", meta.name, meta.version);
}
```

## Calling Java Methods from Rust

```rust
for java_mod in loader.java_mods() {
    // void method with no args
    java_mod.call_method("onTick", "()V", &[])?;

    // void method taking an int
    java_mod.call_method("setValue", "(I)V", &[jni::objects::JValue::Int(42)])?;
}
```

### Common JNI Signatures

| Signature | Meaning |
|-----------|---------|
| `()V` | No params, void return |
| `(I)V` | `int` param, void return |
| `()I` | No params, `int` return |
| `(Ljava/lang/String;)V` | `String` param, void return |
| `([I)V` | `int[]` param, void return |

## Troubleshooting

| Error | Cause | Fix |
|-------|-------|-----|
| "Failed to create JVM" | JDK not found | Set `JAVA_HOME` and update library path |
| "Failed to find class" | Wrong class name format | Use `/` separators: `com/example/MyMod` |
| "Failed to call method" | Wrong signature | Check method signature with `javap -p -s MyMod.class` |

## See Also

- [Installation — Java Setup](/installation)
- [JNI Integration Guide](/jni-integration)
- [Forge Compatibility](/modding/forge)
- [Mod Development Guide](/modding/development)
