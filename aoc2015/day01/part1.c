#include <stdio.h>

int main(int argc, char const *argv[]) {
    if (argc < 2) {
        printf("usage: ./part1 <input_file>\n");
        return -1;
    }

    FILE *input = fopen(argv[1], "r");
    int floor = 0;
    char c;

    while (fscanf(input, "%c", &c) > 0) {
        if (c == '(') floor++;
        else if (c == ')') floor--;
    }

    fclose(input);

    printf("%d\n", floor);

    return 0;
}
