# Gosub MacOS Client
This is a proof of concept and NOT a functional client.

Think of this as a sandbox as we develop the [Gosub engine](https://github.com/gosub-browser/gosub-engine) and figure things out.

# Building
This uses the [Gosub engine](https://github.com/gosub-browser/gosub-engine) as a submodule. When cloning the repository, please use:
```text
git clone git@github.com:gosub-browser/gosub-client-macos.git --recurse-submodule
```

Then run `make` which will compile the Gosub engine, its bindings, and export the headers and libs to this repository's relevant directories. After this, you should be able to open the Xcode project and build successfully.

# Updating
If the C API has changed/updated in the engine, use `make update` and `make` to update the bindings.

# Preview (as of 6 Dec 2023)
![switch tab colour](https://github.com/gosub-browser/gosub-client-macos/assets/49159969/95888dff-128b-479e-a08d-3bd423c78316)
