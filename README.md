# Gosub MacOS Client
This is a proof of concept and NOT a functional client.

Think of this as a sandbox as we develop the [Gosub Engine](https://github.com/gosub-browser/gosub-engine) and figure things out.

# Preview (as of 18 Nov 2023)
<img width="1025" alt="image" src="https://github.com/Kiyoshika/gosub-client-macos/assets/49159969/9886003a-99aa-4dfd-bf15-4927ec95b413">

# Architecture
NOTE: as this client develops, this section will be expanded.

## Browser Tabs and HTML Parsing
The client contains **browser tabs** (starting with one default) which each hold their own `DocumentHandle` from the gosub engine. Initially, the document is blank. When a user enters an address (or path to local html file), it will take that HTML, parse it and build the document tree (once) and store it. That way, whenever switching between tabs, it's not reparsing the entire HTML (although this may need to change for more complex sites.)

## Rendering
For the actual rendering, we initialize a `RenderCursor` to (0, 0) in the render window and a `DocumentHandle` is taken from the selected browser tab and traversed in tree order. A stack of `RenderItem`s is created while traversing the tree. A new `RenderItem` is created when encountering an `Element` node type and pushed onto the stack. If we encounter something like a `Text` node next, we will use that `RenderItem` on the stack and append the text into the element's body (if applicable, such as a `<p>` element, otherwise ignored.) Each `RenderItem` has their own types such as `Heading1`, `Paragraph`, etc. which contain styles and default font sizes. Whenever encountering a new `Element` node (or at the end of the tree) we will draw the content to the screen at the current `RenderCursor` position and adjust the cursor according to the size of the item in preparation to paint the next item. NOTE: eventually this should be handled by a `RenderTree` in the gosub engine itself but we are still far away from that, so in the meantime we are doing some of the math in the client itself.

Each time a document is rendered, the previous stack of `RenderItem`s is iterated and each drawn item is removed from the view effectively "clearing" the screen before rendering the new document tree. I'm not sure of a better way to handle this since there doesn't seem to be a "native" clear screen in a view for appkit.

# What to do?
We are trying to implement various features according to our wish list: https://github.com/gosub-browser/gosub-engine/wiki/Browser-features

If this client becomes mature enough and meets our needs, we can possibly transfer this repo to the official Gosub org.
