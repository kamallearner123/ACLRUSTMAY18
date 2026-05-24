#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int reader(char *ptr) {
	printf("Data in ptr = %s\n", ptr);
	int flag = 10;
	if (flag == 0 )
		free(ptr);
	return 0;
}

int main()
{
	char *data = NULL;
	data = malloc(10);
	memcpy(data, "hello", 5);

	reader(data); // giving ownership to reader

	//free(data);

    return 0;
}
