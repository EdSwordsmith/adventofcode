#include <stdio.h>

int main(int argc, char const *argv[]) {
    if (argc < 2) {
        printf("usage: ./part1 <input_file>\n");
        return -1;
    }

    FILE *input = fopen(argv[1], "r");
    int floor = 0;
    int i = 1;
    char c;

    while (fscanf(input, "%c", &c) > 0) {
        if (c == '(') floor++;
        else if (c == ')') floor--;

        if (floor == -1) break;

        i++;
    }

    fclose(input);

    printf("Part 1: %d\n", floor);
    printf("Part 2: %d\n", i);

    return 0;
}
