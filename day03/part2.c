#include <stdio.h>

#define ROW_SIZE 32
#define SLOPES 5

int main(int argc, char const *argv[])
{
    FILE *file = fopen("input.txt", "r");

    int slopesx[SLOPES] = {1, 3, 5, 7, 1};
    int slopesy[SLOPES] = {1, 1, 1, 1, 2};

    int posx[SLOPES] = {0};
    int posy = 0;

    int trees[SLOPES] = {0};
    char row[ROW_SIZE];

    while (fscanf(file, "%s", row) > 0)
    {
        for (int i = 0; i < SLOPES; i++)
        {
            if (posy % slopesy[i] == 0) 
            {
                if (row[posx[i] % (ROW_SIZE - 1)] == '#')
                    trees[i] += 1;
                posx[i] += slopesx[i];
            }
        }
        
        posy++;
    }

    fclose(file);

    long product = 1;
    for (int i = 0; i < SLOPES; i++)
    {
        product = product * trees[i];
        printf("Trees for slope %d right, %d down: %d\n", slopesx[i], slopesy[i], trees[i]);
    }
    printf("%ld\n", product);

    return 0;
}
