/* "Hello, wolrd!" C version. */

#include <string.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
	const char *buf = "Hello, world!\n";
	unsigned int len = strlen(buf);

	write(1, buf, len);
	exit(0);
}
