🌌 **hack-tui** (working name)
=============================

> **Declarative UIs for Rust** – build once, run in the terminal using Ratatui **and** the browser.
> Inspired by [Textual](https://github.com/Textualize/textual) for Python and React for the web.

* * *

🚀 Vision
---------

Rust has amazing TUI crates (`ratatui`, `crossterm`, etc.) and great GUI/web frameworks (`iced`, `leptos`, etc.), but nothing unifies them in a **declarative, component-based model**.

**hack-tui** aims to change that:

*   Write declarative Rust code (React/SwiftUI style).
    
*   Render **identically** in the terminal (via Ratatui) and the web (via WASM).
    
*   Compose UIs from reusable components.
    
*   Empower both CLI lovers and web developers with the same ergonomic API.
    

* * *

✨ Example (imaginary, for now)
------------------------------

rust

Copy code

`fn main() {     App::new()         .view(             VStack::new()                 .child(Label::new("Hello, world!"))                 .child(Button::new("Click me", |ctx| {                     ctx.log("Button clicked!");                 }))         )         .run(); }`

**Terminal:**

css

Copy code

`┌──────────────────────┐ │  Hello, world!       │ │  [ Click me ]        │ └──────────────────────┘`

**Browser (WASM):**

<p align="center"><img src="docs/mockup\_web.png" width="400"></p>

* * *

📍 Roadmap
----------

*    **MVP** – Terminal-only, Ratatui backend
    
*    **State management** – simple signals/hooks
    
*    **Component system** – buttons, inputs, tables
    
*    **Web backend** – WASM + DOM renderer
    
*    **Themes** – consistent styling across backends
    
*    **Ecosystem** – community-built widgets
    

* * *

🤝 Contributing
---------------

This project is in its **very early stages** – more vision than implementation.  
If you:

*   Love Rust
    
*   Love TUIs
    
*   Love the idea of **one codebase → terminal + web**
    

…then come join! Open issues, propose APIs, or hack on an MVP.

* * *

💡 Inspiration
--------------

*   [Textual (Python)](https://github.com/Textualize/textual)
    
*   [Ratatuí (Rust)](https://github.com/ratatui-org/ratatui)
    
*   React
    

* * *

📜 License
----------

MIT — free to use, hack, dream.

* * *

