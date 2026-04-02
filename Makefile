SRC = $(wildcard src/*.rs)
CONFIG = config.json
DATA = output/prevalence.csv
LOG = WARN

.PHONY: all clean

all: $(DATA)
	@cat $(DATA)

$(DATA): $(SRC)
	cargo run -- --output output/ --config $(CONFIG) --log-level=$(LOG)

clean:
	rm -rf output/*
