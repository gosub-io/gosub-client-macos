BINDINGS_DIR := gosub-engine/crates/gosub-bindings

# build the engine and the bindings
build:
	make -C $(BINDINGS_DIR) bindings
	cp -r $(BINDINGS_DIR)/include/* include/
	cp -r $(BINDINGS_DIR)/lib/* lib/

# update the engine submodule (if the C API has changed)
update:
	git submodule update --remote
