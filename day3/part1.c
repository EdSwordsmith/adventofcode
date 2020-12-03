#include <stdio.h>

#define ROW_SIZE 32

int main(int argc, char const *argv[])
{
    FILE *file = fopen("input.txt", "r");

    int posx = 0;
    int trees = 0;
    char row[ROW_SIZE];

    while (fscanf(file, "%s", row) > 0)
    {
        if (row[posx % (ROW_SIZE - 1)] == '#')
            trees++;
        posx += 3;
    }

    fclose(file);

    printf("%d\n", trees);

    return 0;
}
