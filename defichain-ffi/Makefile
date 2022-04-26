CARGO ?= cargo
install:
	git submodule update
	./depend/ain/make.sh build
build :
	$(CARGO) build &> build.log
	tail -n 100 build.log

clean :
	./depend/ain/make.sh clean
	$(CARGO) clean
