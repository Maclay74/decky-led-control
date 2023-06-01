# This is the default target, which will be built when 
# you invoke make
.PHONY: all
all: hello

# This rule tells make how to build hello from hello.cpp
hello:
	mkdir -p ../bin
	docker run --rm -v `pwd`:/backend decky-led-control
	cp ./bin/backend ../bin/backend
	cp ./ectool/ectool ../bin/ectool

# This rule tells make to delete hello and hello.o
.PHONY: clean 
clean:
	rm -f hello