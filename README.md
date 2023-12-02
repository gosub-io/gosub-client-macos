# Gosub MacOS Client
This is a proof of concept and NOT a functional client.

Think of this as a sandbox as we develop the [Gosub engine](https://github.com/gosub-browser/gosub-engine) and figure things out.

# Building
This uses the [Gosub engine](https://github.com/gosub-browser/gosub-engine) as a submodule. When cloning the repository, please use:
```text
git clone git@github.com:gosub-browser/gosub-client-macos.git --recurse-submodule
```

Then run `make` which will compile the Gosub engine, its bindings, and export the headers and libs to this repository's relevant directories. After this, you should be able to open the Xcode project and build successfully.

# Preview (as of 25 Nov 2023)
![image](https://github.com/gosub-browser/gosub-client-macos/assets/49159969/ed546d09-5893-4736-9694-6f0651607b5a)
