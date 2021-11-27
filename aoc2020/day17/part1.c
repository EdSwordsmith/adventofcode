#include <stdio.h>
#include <string.h>

#define MIN(X, Y) (((X) < (Y)) ? (X) : (Y))
#define MAX(X, Y) (((X) > (Y)) ? (X) : (Y))

#define CYCLES 6
#define INPUT_WIDTH 8
#define WIDTH INPUT_WIDTH + 2 * CYCLES
#define HEIGHT 2 * CYCLES + 1

typedef enum { ACTIVE, INACTIVE } pos_t;

pos_t positions[HEIGHT][WIDTH][WIDTH];

pos_t get_position(int x, int y, int z)
{
    if (x < 0) x += WIDTH;
    if (y < 0) y += WIDTH;
    if (z < 0) z += WIDTH;

    return positions[z][x][y];
}

void set_position(pos_t map[HEIGHT][WIDTH][WIDTH], int x, int y, int z, pos_t state)
{
    if (x < 0) x += WIDTH;
    if (y < 0) y += WIDTH;
    if (z < 0) z += WIDTH;

    map[z][x][y] = state;
}

void fill_positions(FILE *file)
{
    char input[INPUT_WIDTH + 1];
    int y = 0;

    while (fscanf(file, "%s", input) != EOF)
    {
        for (int x = 0; x < INPUT_WIDTH; x++)
        {
            if (input[x] == '#')
                set_position(positions, x, y, 0, ACTIVE);
            else
                set_position(positions, x, y, 0, INACTIVE);
        }

        y++;
    }
}

int get_adjacent_active(int x, int y, int z)
{
    int count = 0;

    for (int i = x - 1; i <= x + 1; i++)
    {
        for (int j = y - 1; j <= y + 1; j++)
        {
            for (int k = z - 1; j < z + 1; k++)
            {
                if (i == x && j == y && k == z)
                    continue;
                
                if (get_position(i, j, k) == ACTIVE)
                    count++;
            }
        }
    }
    
    return count;
}

pos_t position_tick(int x, int y, int z)
{
    int adjacent = get_adjacent_active(x, y, z);
    pos_t state = get_position(x, y, z);

    if (state == INACTIVE && adjacent == 3)
        state = ACTIVE;
    else if (state == ACTIVE && adjacent != 2 && adjacent != 3)
        state = INACTIVE;

    return state;
}

void positions_tick()
{
    pos_t new_state[HEIGHT][WIDTH][WIDTH];

    for (int i = 0; i < WIDTH; i++)
    {
        for (int j = 0; j < WIDTH; j++)
        {
            for (int k = 0; k < HEIGHT; k++)
            {
                pos_t state = position_tick(i, j, k);
                set_position(new_state, i, j, k, state);
            }
        }
    }

    // Copy new state into the positions
    memcpy(positions, new_state, sizeof(positions));
}

int count_active()
{
    int count = 0;

    for (int i = 0; i < WIDTH; i++)
    {
        for (int j = 0; j < WIDTH; j++)
        {
            for (int k = 0; k < HEIGHT; k++)
            {
                if (positions[k][i][j] == ACTIVE)
                    count++;
            }
        }
    }

    return count;
}

int main(int argc, char const *argv[])
{
    FILE *file = fopen("example.txt", "r");
    fill_positions(file);
    fclose(file);

    for (int i = 0; i < CYCLES; i++)
    {
        positions_tick();
    }

    printf("%d\n", count_active());

    return 0;
}
