#!/usr/bin/python3
"""
--- Part Two ---
Now, you're ready to check the image for sea monsters.
The borders of each tile are not part of the actual image; start by removing
them.
In the example above, the tiles become:
.#.#..#. ##...#.# #..#####
###....# .#....#. .#......
##.##.## #.#.#..# #####...
###.#### #...#.## ###.#..#
##.#.... #.##.### #...#.##
...##### ###.#... .#####.#
....#..# ...##..# .#.###..
.####... #..#.... .#......
#..#.##. .#..###. #.##....
#.####.. #.####.# .#.###..
###.#.#. ..#.#### ##.#..##
#.####.. ..##..## ######.#
##..##.# ...#...# .#.#.#..
...#..#. .#.#.##. .###.###
.#.#.... #.##.#.. .###.##.
###.#... #..#.##. ######..
.#.#.### .##.##.# ..#.##..
.####.## #.#...## #.#..#.#
..#.#..# ..#.#.#. ####.###
#..####. ..#.#.#. ###.###.
#####..# ####...# ##....##
#.##..#. .#...#.. ####...#
.#.###.. ##..##.. ####.##.
...###.. .##...#. ..#..###
Remove the gaps to form the actual image:
.#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###
Now, you're ready to search for sea monsters! Because your image is monochrome,
a sea monster will look like this:
                  #
#    ##    ##    ###
 #  #  #  #  #  #
When looking for this pattern in the image, the spaces can be anything; only
the # need to match. Also, you might need to rotate or flip your image before
it's oriented correctly to find sea monsters. In the above image, after
flipping and rotating it to the appropriate orientation, there are two sea
monsters (marked with O):
.####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.O#..
#.O.##.OO#.#.OO.##.OOO##
..#O.#O#.O##O..O.#O##.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.##O###.
.O##.#OO.###OO##..OOO##.
..O#.O..O..O.#O##O##.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#
Determine how rough the waters are in the sea monsters' habitat by counting the
number of # that are not part of a sea monster. In the above example, the
habitat's water roughness is 273.
How many # are not part of a sea monster?
"""

import collections
import re
import sys


# These are regexps!  . matches anything, # matches #
MONSTER = [
    '..................#.',
    '#....##....##....###',
    '.#..#..#..#..#..#...',
]


class Tile(list):
    id = None

    def __init__(self, *args, id=None):
        super().__init__(*args)
        self.id = id

    @property
    def top(self):
        return self[0]

    @property
    def right(self):
        return ''.join(row[-1] for row in self)

    @property
    def bottom(self):
        return self[-1][::-1]

    @property
    def left(self):
        return ''.join(row[0] for row in self[::-1])

    def __str__(self):
        return '\n'.join(self)


def tile_edges(tile):
    return [tile.top, tile.right, tile.bottom, tile.left]


def rotate(tile):
    return Tile([''.join(row) for row in zip(*tile)][::-1], id=tile.id)


def flip_top_bottom(tile):
    return Tile(tile[::-1], id=tile.id)


def flip_left_right(tile):
    return Tile([row[::-1] for row in tile], id=tile.id)


flip = flip_top_bottom  # when we don't care


def remove_border(tile):
    return Tile([
        row[1:-1] for row in tile[1:-1]
    ], id=tile.id)


def extend_image(image, extra):
    for n, row in enumerate(extra, len(image) - len(extra)):
        image[n] += row


def parse_tiles(f):
    tiles = {}
    for line in f:
        tile = Tile(id=int(line.partition(':')[0].split()[-1]))
        for line in f:
            line = line.strip()
            if not line:
                break
            tile.append(line)
        tiles[tile.id] = tile
    return tiles


def count_edges(tiles):
    edges = collections.Counter()
    for tile in tiles:
        edges.update(tile_edges(tile))
    return edges


def count_unique_edges(tile, edges):
    unique = 0
    for edge in tile_edges(tile):
        assert edges[edge] in (1, 2)
        if edges[edge] == 1:
            unique += 1
    return unique


def find_corner(tiles, edges):
    for tile in tiles:
        if count_unique_edges(tile, edges) == 2:
            return tile
    raise LookupError('could not find corner tile')


def all_sides(tile, flip=flip):
    for side in range(2):
        yield tile
        tile = flip(tile)


def all_rotations(tile):
    for rotation in range(4):
        yield tile
        tile = rotate(tile)


def all_orientations(tile):
    for tile in all_rotations(tile):
        yield from all_sides(tile)


def orient_top_left_corner(tile, edges):
    # This is very brute force and I'm sure I could have recorded the
    # orientation/side back when I was computing edge signatures.
    # Note that there are two possible orientations for a top-left corner,
    # the one this returns is arbitrary
    for tile in all_orientations(tile):
        if edges[tile.left] == 1 and edges[tile.top] == 1:
            return tile
    raise ValueError('this is not a corner tile')


def orient_top_edge(tile, edges):
    # Note: we need to keep the left edge on the left, so no rotations!
    for tile in all_sides(tile, flip=flip_top_bottom):
        if edges[tile.top] == 1:
            return tile
    raise ValueError('this is not an edge tile')


def orient_top_edge_to_match(tile, top_tile):
    # Note: we need to keep the left edge on the left, so no rotations!
    for tile in all_sides(tile, flip=flip_top_bottom):
        if tile.top == top_tile.bottom[::-1]:
            return tile
    raise ValueError(f'this tile ({tile.id}) does not match {top_tile.id}')


def orient_left_edge(tile, edges):
    # Note: we need to keep the top edge on the top!
    for tile in all_sides(tile, flip=flip_left_right):
        if edges[tile.left] == 1:
            return tile
    raise ValueError('this is not an edge tile')


def find_adjacent_on_right(tile, tiles, used):
    # Note that there are two possible orientations for the adjacent tile;
    # the one this returns is arbitrary!
    for candidate in tiles:
        if candidate.id not in used:
            for oriented in all_orientations(candidate):
                if tile.right == oriented.left[::-1]:
                    return oriented
    return None


def find_adjacent_on_bottom(tile, tiles, used):
    for candidate in tiles:
        if candidate.id not in used:
            for oriented in all_orientations(candidate):
                if tile.bottom == oriented.top[::-1]:
                    return oriented
    return None


def monster_matches_at(x, y, image):
    for y, pattern in enumerate(MONSTER, y):
        if not re.match(pattern, image[y][x:]):
            return False
    return True


def find_monsters(image):
    for y, row in enumerate(image[:-len(MONSTER)]):
        for x, col in enumerate(row[:-len(MONSTER[0])]):
            if monster_matches_at(x, y, image):
                yield (x, y)


def count_hashes(image):
    return sum(row.count('#') for row in image)


if __name__ == "__main__":
    with open("input.txt" if len(sys.argv) < 2 else sys.argv[1]) as f:
        tiles = parse_tiles(f).values()

    if '-v' in sys.argv:
        print(f"{len(tiles)} tiles")

    edges = count_edges(tiles)
    edges += count_edges(map(flip, tiles))

    if '-v' in sys.argv:
        print(f"{len(edges)} unique edges, counting flipping")

    corner = find_corner(tiles, edges)
    corner = orient_top_left_corner(corner, edges)

    if '-v' in sys.argv:
        print(f"Starting from tile {corner.id}")

    image = remove_border(corner)

    used = {corner.id}
    start = corner
    tiled = [[corner]]

    tile = start
    while adjacent := find_adjacent_on_right(tile, tiles, used):
        used.add(adjacent.id)
        if '-v' in sys.argv:
            print(f"Adjacent tile is {adjacent.id}")

        adjacent = orient_top_edge(adjacent, edges)
        extend_image(image, remove_border(adjacent))
        tiled[-1].append(adjacent)
        tile = adjacent

    while adjacent := find_adjacent_on_bottom(start, tiles, used):
        used.add(adjacent.id)
        if '-v' in sys.argv:
            print(f"Next row starts with is {adjacent.id}")

        adjacent = orient_left_edge(adjacent, edges)
        image += remove_border(adjacent)
        start = adjacent
        tiled.append([adjacent])

        tile = start
        while adjacent := find_adjacent_on_right(tile, tiles, used):
            used.add(adjacent.id)
            if '-v' in sys.argv:
                print(f"Adjacent tile is {adjacent.id}")

            top_tile = tiled[-2][len(tiled[-1])]
            adjacent = orient_top_edge_to_match(adjacent, top_tile)
            extend_image(image, remove_border(adjacent))
            tiled[-1].append(adjacent)
            tile = adjacent

    if '-v' in sys.argv:
        print(*image, sep='\n')
        print("Tiles:")
        for row in tiled:
            print(*[tile.id for tile in row], sep='\t')

    assert len(used) == len(tiles)
    assert all(len(row) == len(image[0]) for row in image if row)

    for n, image in enumerate(all_orientations(image)):
        monsters = list(find_monsters(image))
        if '-v' in sys.argv:
            print(f"Orientation {n}: {len(monsters)} monsters")
        if monsters:
            hashes = count_hashes(image)
            print(hashes - count_hashes(MONSTER) * len(monsters))