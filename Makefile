SRC = $(wildcard src/*.rs)
DATA = output/creation.csv

all: $(DATA)
	@cat $(DATA)

$(DATA): $(SRC)
	cargo run -- --output output/
