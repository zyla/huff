CFLAGS = -O2

encode: encode.c

dump: encode
	echo -n "DEADBEEFCAFE" | ./encode | xxd -b
