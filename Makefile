.PHONY: build bench report figures clean

build:
	cargo build --release

bench: build
	./target/release/benchmark

report:
	cd docs/informe && latexmk -pdf -interaction=nonstopmode main.tex

figures: bench report

clean:
	cd docs/informe && latexmk -C
