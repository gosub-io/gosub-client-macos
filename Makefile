BINDINGS_DIR := gosub-engine/crates/gosub-bindings

build:
	make -C $(BINDINGS_DIR) bindings
	cp -r $(BINDINGS_DIR)/include/* include/
	cp -r $(BINDINGS_DIR)/lib/* lib/
