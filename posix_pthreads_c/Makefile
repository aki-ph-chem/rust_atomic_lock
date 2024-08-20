CC = gcc
file = ""

ver = 11
OPT=2
CXX_FLAGS = -O${OPT} -pthread -Wall -std=c${ver}

${file}.out : ${file}.c
	${CC} -o $@ $< ${CXX_FLAGS}

run: ${file}.out
	./${file}.out

clean:
	rm ${file}.out

clean_all:
	rm *.out

.PHONY: run clean clean_all
