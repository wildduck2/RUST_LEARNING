#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *dangle() {
  char *s = malloc(6);
  strcpy(s, "hello");

  free(s);

  return s; // hello
  //  -> [ | | | | | ]
}

int main() {
  char *str = dangle();
  printf("Dangling: %s\n", str);
}
