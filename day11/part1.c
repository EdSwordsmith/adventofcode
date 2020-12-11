#include <stdio.h>
#include <string.h>

#define MIN(X, Y) (((X) < (Y)) ? (X) : (Y))
#define MAX(X, Y) (((X) > (Y)) ? (X) : (Y))

#define COLS 92
#define ROWS 99

typedef enum { FLOOR, EMPTY_SEAT, OCCUPIED_SEAT } pos_t;

pos_t positions[ROWS * COLS];

int get_index(int row, int col)
{
    return row * COLS + col;
}

void fill_positions(FILE *file)
{
    char input[COLS + 1];
    int row = 0;

    while (fscanf(file, "%s", &input) != EOF)
    {
        for (int i = 0; i < COLS; i++)
        {
            int index = get_index(row, i);
            if (input[i] == 'L')
                positions[index] = EMPTY_SEAT;
            else
                positions[index] = FLOOR;
        }

        row++;
    }
}

int get_adjacent_occupied(int row, int col)
{
    int count = 0;

    for (int i = MAX(row - 1, 0); i <= MIN(row + 1, ROWS - 1); i++)
    {
        for (int j = MAX(col - 1, 0); j <= MIN(col + 1, COLS - 1); j++)
        {
            if (i == row && j == col)
                continue;

            int index = get_index(i, j);
            if (positions[index] == OCCUPIED_SEAT)
                count++;
        }
    }
    
    return count;
}

pos_t position_tick(int row, int col)
{
    int index = get_index(row, col);
    int adjacent = get_adjacent_occupied(row, col);
    pos_t state = positions[index];

    if (state == EMPTY_SEAT && adjacent == 0)
        state = OCCUPIED_SEAT;
    else if (state == OCCUPIED_SEAT && adjacent >= 4)
        state = EMPTY_SEAT;

    return state;
}

int positions_tick()
{
    int changed = 0;
    pos_t new_state[ROWS * COLS];

    for (int i = 0; i < ROWS; i++)
    {
        for (int j = 0; j < COLS; j++)
        {
            pos_t state = position_tick(i, j);
            int index = get_index(i, j);
            new_state[index] = state;
            if (positions[index] != state)
                changed = 1;
        }
    }

    // Copy new state into the positions
    memcpy(positions, new_state, sizeof(positions));
    
    return changed;
}

int count_occupied()
{
    int count = 0;

    for (int i = 0; i < ROWS * COLS; i++)
    {
        if (positions[i] == OCCUPIED_SEAT)
            count++;
    }

    return count;
}

int main(int argc, char const *argv[])
{
    FILE *file = fopen("input.txt", "r");
    fill_positions(file);
    fclose(file);

    while (positions_tick()); 

    printf("%d\n", count_occupied());

    return 0;
}
