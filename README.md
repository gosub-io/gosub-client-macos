# Gosub MacOS Client
This is a proof of concept and NOT a functional client.

Think of this as a sandbox as we develop the [Gosub Engine](https://github.com/gosub-browser/gosub-engine) and figure things out.

# Preview (as of 18 Nov 2023)
<img width="1025" alt="image" src="https://github.com/Kiyoshika/gosub-client-macos/assets/49159969/9886003a-99aa-4dfd-bf15-4927ec95b413">

# Architecture
NOTE: as this client develops, this section will be expanded.

## Browser Tabs and HTML Parsing
The client contains **browser tabs** (starting with one default) which each hold their own `RenderTree` from the gosub engine. Initially, the tree is empty.

In the near future, when a user enters an address (or path to local html file), it will take that HTML, parse it and build the render tree (once) and store it. That way, whenever switching between tabs, it's not reparsing the entire HTML (although this may need to change for more complex sites.)

Currently, all browser tabs store dummy HTML for testing.

## Rendering
Given a `RenderTree` (e.g., after selecting a tab), it will traverse the tree and check the associated node type (e.g., `TextNode`) which will determine which appkit components to use (e.g., `Label` in the case of a `TextNode`). The nodes of a render tree have different properties such as `margin`, `padding`, `font`, `font size`, etc. to help draw the appropriate sizes on the screen.

Currently the render tree is very basic and maintains a `Position` which determines where to draw the next item. The current/next item's margins and font sizes are taken into account when updating this `Position` to place the next item.

# What to do?
We are trying to implement various features according to our wish list: https://github.com/gosub-browser/gosub-engine/wiki/Browser-features

We are also trying to implement a render tree in the engine itself to prevent the client from having to do all the layout calculations.

# (Main) Next Steps
In no particular order...
* Consider switching to [core-foundation](https://github.com/servo/core-foundation-rs)
  * Even though [cacao](https://github.com/ryanmcgrath/cacao) is very nice to use, it's unfortunately just too incomplete for what we want to do.
  * This means the entire UI would have to be rewritten but since there's not much yet it's better to make a decision sooner rather than later
* Consider writing the UI in SwiftUI and interact with the engine through HTTP
  * This is an alternative if the other bindings are too difficult/inconvenient to use, we would send information from the gosub-engine regarding what to render through HTTP which is read by Swift and used to build the objects.
* Consider using FFI to expose Rust to Swift
  * This is an alternative to using HTTP and the overhead of serializing/deserializing JSON or some other format between the languages.
  * We would likely need separate "FFI-specific" wrappers that "convert" the Rust structs into C-friendly structs (or at least expose certain pointers to Swift)
  * We primarily need to expose just the `RenderTree` (for now) and maybe some methods from it
* introduce box layout in render tree
* integrate CSSOM to compute styles when it's built
* support more element types
