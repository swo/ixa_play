SRC = $(wildcard src/*.rs)
DATA = output/prevalence.csv
LOG = WARN

all: $(DATA)
	@cat $(DATA)

$(DATA): $(SRC)
	cargo run -- --output output/ --log-level=$(LOG)
