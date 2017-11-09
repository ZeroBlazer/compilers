CC = gcc
CFLAGS = -Wall
LIBS = -lfl

EXECS = scanner
OBJS = lex.yy

all: ${EXECS}

lex.yy:
	lex tokens.l

scanner:
	${CC} 

clean:
	rm -f ${EXECS} ${OBJS}
	@echo "all cleaned up!"