CC=gcc
CFLAGS=-O2
LDFLAGS=-s

OBJS=bench-collatz.o
TARGET=bench-collatz-C

ALL: $(OBJS)
	$(CC) $(LDFLAGS) $(OBJS) -o $(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)
