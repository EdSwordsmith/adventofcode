#include <stdio.h>
#include <stdlib.h>

typedef enum
{
    WEST = 0,
    NORTH = 1,
    EAST = 2,
    SOUTH = 3
} direction_t;

typedef struct
{
    int ns; // North - South
    int ew; // East - West
    direction_t dir;
} ship_t;

ship_t ship;

void handle_command(const char command, const int arg)
{
    int rotate;

    switch (command)
    {
    case 'N':
        ship.ns += arg;
        break;
    case 'S':
        ship.ns -= arg;
        break;
    case 'E':
        ship.ew += arg;
        break;
    case 'W':
        ship.ew -= arg;
        break;
    case 'R':
        rotate = arg / 90;
        ship.dir = (ship.dir + rotate) % 4;
        break;
    case 'L':
        rotate = arg / 90;
        ship.dir = (ship.dir + 4 - rotate) % 4;
        break;
    case 'F':
        switch (ship.dir)
        {
        case NORTH:
            handle_command('N', arg);
            break;
        case SOUTH:
            handle_command('S', arg);
            break;
        case EAST:
            handle_command('E', arg);
            break;
        case WEST:
            handle_command('W', arg);
            break;
        }
    }
}

int main(int argc, char const *argv[])
{
    ship.ns = 0;
    ship.ew = 0;
    ship.dir = EAST;

    FILE *file = fopen("input.txt", "r");
    char command;
    int arg;

    while (fscanf(file, "%c%d", &command, &arg) != EOF)
    {
        handle_command(command, arg);
    }

    printf("%d %d -> %d\n", ship.ew, ship.ns, abs(ship.ns) + abs(ship.ew));

    fclose(file);

    return 0;
}
