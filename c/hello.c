/* "Hello, wolrd!" C version. */

#include <stdlib.h>
#include <unistd.h>

int main() {
	const char buf[] = "Hello, world!\n";
	const unsigned int len = sizeof(buf);

	write(1, buf, len);
	exit(0);
}
