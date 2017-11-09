CC = gcc
CFLAGS = -Wall
LIBS = -lfl

EXECS = scanner
OBJS = lex.yy.c

all: ${EXECS}

lex.yy.c: tokens.l
	lex tokens.l

scanner: lex.yy.c
	${CC} ${CFLAGS} -o scanner ${OBJS} ${LIBS}

run: all
	./scanner

clean:
	rm -f ${EXECS} ${OBJS}
	@echo "all cleaned up!"