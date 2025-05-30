all: examples/Add.class examples/Hello.class

examples/%.class: examples/%.java
	javac -d examples $<

.PHONY: all
