# GDFABRIC

The Rust library that connects to Geometry Dash live editor via WebSocket, allowing you to programmatically add, modify, and remove game objects in real time

<img src="https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white" height=28 />

It was created as a more robust alternative to tools like [NeditGD](https://github.com/Boris-Filin/NeditGD), focusing on:

* High throughput – minimal overhead and fast serialisation for handling hundreds or thousands of objects per frame

* Commercial‑grade stability – designed for projects where reliability and predictable performance are essential

* Full control – every field of a game object is exposed (position, rotation, scale, colour, HSV, groups, etc.)

* Easy integration – a simple API to send commands and receive responses over a WebSocket connection

Use it for automated level generation, real‑time visualisation tools, or any workflow that requires manipulating Geometry Dash levels from external code

# Usage
To connect via a websocket, install WSLiveEditor in [Geode](https://geode-sdk.org/mods/iandyhd3.wsliveeditor) or from [GitHub](https://github.com/iAndyHD3/WSLiveEditor)

Using a websocket to add one object and the text `Hello, World!`:

```rust
use gdfabric::*

fn main() -> Result<(), gdfabric::Error> {
    let mut editor = Editor::load_ws()?;
    
    let obj1 = GameObject::new();
    
    let mut obj2 = TextGameObject::from(String::from("Hello, World!"), 5);
    obj2.pos = Point::new(105.0, 105.0);
    
    editor.add_objects(vec![obj1, obj2]);
    
    editor.save()?;
    
    Ok(())
}
```
