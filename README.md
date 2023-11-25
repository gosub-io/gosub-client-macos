# Gosub MacOS Client
This is a proof of concept and NOT a functional client.

Think of this as a sandbox as we develop the [Gosub Engine](https://github.com/gosub-browser/gosub-engine) and figure things out.

# Prerequisites
If you plan on building this project from source, you must follow these steps to get started:
1. Install XCode
2. Clone the engine
    * `git clone git@github.com:gosub-browser/gosub-engine.git`
    * `cd gosub-engine/gosub-bindings`
    * `cargo build`
    * `make bindings`
3. Clone this repo
    * `git clone git@github.com:gosub-browser/gosub-client-macos.git`
    * copy the contents of `gosub-engine/gosub-bindings/include` to `gosub-client-macos/include`
    * copy the contents of `gosub-engine/gosub-bindings/lib` to `gosub-client-macos/lib`

You should now be able to double click the xcode project file and build successfully...hopefully.

# Preview (as of 25 Nov 2023)
![image](https://github.com/gosub-browser/gosub-client-macos/assets/49159969/ed546d09-5893-4736-9694-6f0651607b5a)
