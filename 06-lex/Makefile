CC = gcc
CFLAGS = -Wall
LIBS = -lfl

EXEC = scanner
OBJS = lex.yy

all: ${EXEC}

lex.yy:
	lex tokens.l

scanner:
	${CC} 