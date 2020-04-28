CFLAGS = -O2

encode: encode.c

dump: encode
	echo "Reading package lists" | ./encode | xxd
