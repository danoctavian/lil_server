RUSTC ?= rustc

#add your targets here
all: lil_server bin/main

run: bin/foomain
	./bin/foomain

.PHONY : lil_server 

lil_server:
	$(RUSTC) --out-dir bin -L bin src/lil_server/lil_server.rc

bin/main: src/main/main.rc src/main/*.rs lil_server
	$(RUSTC) -L bin -o $@ $<
