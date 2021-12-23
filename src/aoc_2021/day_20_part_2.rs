use crate::aoc_2021::day_20_part_1::day_20_part_1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_20_part_2() {
        let input =
r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;

        let ans = day_20_part_1(&input, 50);

        assert_eq!(ans, 3351);
    }

    #[test]
    fn test_day_20_part_2() {
        let input =
r#"##..#..##.#..#...####..#.#..##.###.......###.###..#..######...#.#....#...##.###.......#.#..####.#..#......#.#...##.#..##.#.##.#..####.#..##.....#.....#....###.#.#....##..##.##.####.##.###..###..####..#.#.#####..#####.####..######.##....#..#.#...#..##..###....#...#####.#..###...##..#.######..#.##.#....######.....###..##..######.....#..##.##..#.###...####.#...#.##.#.#....####.##.###...####.#.#.#.###..##..##....#.#.#..###.##.###..##.#...##.#.#####.#..#...####..###.###...###.#..#..##.#...#....##.#....##...##.#.

#..##.#..#.##..#.#.##...##.####.....#..#.#######.#.####.##.#.#......#.#..####.#..#####.#...###..###.
.#...#...#..###.#..#.......########..###...###.#....######.#.#..##..#..###..####.#.#..#..#..#.#.#..#
#.##....#.###..#.#....###..#....##.##..#####.###.#.#.....#.###...#.#..#.###.##..##..#..#..#########.
.#######.##.#########.#..###...#..#.###...##..#.#########..#.##.#.##..###..##.#.###..##.#.###.#..#..
#......####.#.....###..#...####.####.###...#.##.##.##.....#.#..#....#..###.....##.#.#...###.##.##.##
...##.#####.#.#..##...##....#.#..#.#...###..#.#.#..#.#..##..#.##.#.##.####.#..#....##.##..##.##.###.
#.#...##..##..#...##..##.######.#..##..#.##.#########..##.#.#..#..#...#########..#.###..####..#...##
#####..####..###...##...#.##.##.#..#.#....#....#.#####....##..###.#####.##..#.##...###..##...#.#.###
.#.##.####.....#.#.##.####.##.####.######..##..#.#..###..#.##.#####.#.####.#.##.#.#......########.#.
..#......##...##...#..###....#....####..###.##..####...#.#..#.#...#...##.##.#...#.#........#........
.....#.#.#..#..#.####.#...####.#####.###.#.#.##.#.#.#.......#.#..##.#.###..##........###.....##..##.
##...#.###...####.#...##..##..##.#.###..##.##..##..###..###.#.##.#.##...#.#..###.##..#....###..#.##.
##.##....##..#.....###.#####.....##..##..###..###.#...##..#..#..###....#...#.##.###.##.##...#.....#.
###.######.#.#....#.#....##...####..#.##....#.##..##.#####.#.......#....#.#.#####.#..###...##.##...#
##.##.###.#..##.###....#.##..........##.##.#...#...#..###......#..#.###..##.#####.###..##.###.###.#.
..##.....#.####...##.....##.#..#....##...#.#...#..###..#..##.###.#...#..#.###.#.....#.#.###.##..#.##
#..#.######.#.#.####.#.#####..#..#..#.##..#..##.#...#.....##..#....#.#..#.##..###..#..##.#########..
..#.#.###.....##.#.##.#.##...###...#######.###.#...#..#.###.#.#..#....##..#.#..##....#.##.#.##....##
#.#.##.#.#.###..#.#..#.####..#....#...#..##..####....#.#...#.#...#.#.....#..###.##.#..#.##...#.#..#.
..###...#..###...##..##....#.....#.#.##.......##..#...##.##.##...####.#..#..##.....##..#..#..#.....#
..#.#....#..###..###..###.....##.#..##.##.###..#.##.#.#...##..#.......#.#....####...#.#####...#..##.
.#.######...#.....#....#.##.#.#.#####.##.#.#.#.#..###...#.#..#..##.###.##.....##.#..#.####.#.....#..
....#.#...##..#..##.###.#######..##...#..##..########..##....#........#..###.##.#.##.......##.#.####
##.#.##..#..#.....##...#.#...###.##.#..#..####....###..#..#....#..#..##....###..#..#..#..###...#....
##..##.#..#.##.#.#####..#.#.##.#.####..#.#.#.#.#..#.#....######.#..##.......###..#.###.#..##..#.##..
###...##.####.#.#..#####.....###.#.####...###.#.#.#....###.####.##.####..##..##..#.#.####...#..#.##.
..##..#..#.###.######..##..####.#......#.#.###.#.#...#....#.#....#.##.#####.....#.##.#.#.#.##.##..#.
#.#..#.#.#.###..##..#####..##....#.###..##.##..#.#..###.....##.#.#.#####........#####.#####.#.#..#.#
..#.##.....#..###...#.......#.#.##..#.#....#####.###.###.########.#.#.###..#..##...#..#..#........##
#..##.#..#....##.#....####.#..##..#......#....##.#...#..##.##.###...#.....##....#.####.#...#.###.##.
###.###.#..####.##..###..##..###..#..##.#.#.####...#..##..#..###.##.#..##..####.#.###..#...##.....#.
##..###...#...#.#.#.##.#####..#..####.#.#...#..#.#.#.#.#.##.####...########..#.###..##.#..#.#######.
######.##.#.#.#.#.######.##.####..####.#.#..#...###.....#.###..#.#.#.###.#......#..#..##..###.###...
#.##.##.###..###.#.#...#.#.#.#....#.#...#.###..####.#.##.....##.##..##....##..#.##...#.........#.###
.#..##....#.#..#.#..#....#.##.####.##.#.#...#.#.##.#..#..#.###..#..####..##......##.#.#....##..#...#
.#..#####...#.#..#.#....#.#.###########..##........##.###.##..#..##...###..##.#####..###.#.##...#...
###..##...#..##.###....#####...#.##...#..#.....#.##.#.####...##...######..#.#.##...########....#.#.#
##..####.##.#...#.#.###..###..#...##...#....##..#.....##.#.###....#..#...#.##...#.###.....###..###.#
#.###.#.##...##....####.#.#.###.#.#.#....#.##.........##..#.###..##..#...#.#.##....#..#.###.#..##...
#....#####..#.#.##..##...#...#.....#.#.#..##.#.##.#...##.###.###...#.###.#.##.####..#.....###..###..
#...#####.###.#....#..#..#..#.#.#..###....#.#.###.##.##.##..##..#..####.#...#.#.#.##....#....#.##.##
.##.....#.......#..##.##..#...##..#...#....#.#......##.#.####.#.#####.##...##.#.....#.#.###.#.####.#
#.###.#..#.###...#.........##..##..#.#.#.#..#.#..#..#.##..#.....##....##..##.#...##..###.#.#.#....#.
#####.####..###.##.###.###..#.##..######..###.##..#..##..#.##.#.#.###...#####.##.#..###.#.##.##.#...
.##.#...#.#.#....##.##...####.##..##.#.#.#.#..##.#.#.##.####....####.#.##..##...#......#.#####.##..#
.####......#.#.#...#.###.##..####.#.#..#.##.#.##...#..##.#.#..#....#.#.#..#..##..##.#...#.#.##.##.#.
.####..###.#.....#..#.##......####....#.#.#...##.#....##.#.#######...#..#..##....#...###..#.##.....#
###......##..####.#..#.#..###....#.##.#......#...#..##..#.#.#.##..##.##..#.#..#######..#..##.##.#...
#.#.#...##.#.#...#........##.#..#.#.#..#..##.#.......#..#.####..###.##..#.##.#....##.##..###..####.#
...####...##...#.##.#.##....#.###..##.###..##...#..#####...###.....###.##.###.##.#..#....#.##.#.#..#
.###.#..#..#...#..#.#.#.##.##..#.##.#.#...##..#......#...#..##.#.#.....#.#.....#.##....#..#....##...
.....#.##..#...###.#..#.....#####..#.....#.#.###.####....#.###.#.####.###.#.##.#.###.#.##.###..#....
.#.#..##....#...#..##.....##.#.#..#.#.#..#.......##.###.###.#.########.#.##.#.....#.###.#..#.###.###
####.#.#.#####.#.####.##.##.######.##..#...###......##...#######.#.##.####..#..##.####...##..##...##
###.#.....#.#####.#.#.########.##..#.#...#.######.###...##..#.#.#####..#.##.#.#.#..#..##..####.#....
###.#.#....#####.####.##.##..#..#.#.###...###.#...##...#.##...#.#.#.#.######..##....##.#.#...####..#
##..#.#.......###.##.#.##.##.###.#.........#.#.#......####.#.######.#####.#.#.##...##.#.#.#......#..
#..##.##.#.....#.#..###..##...#.####.#.####..#.#####.#.##...##...##.#####..#..##..###..#.#..#..###..
.#..#.###.....#.###.#..#...##.##.##..##.#.####..#..##....#....##.##...#.##.....###..#####....#....#.
###.##.#.##.##.##...#####..##......##....##.##.#....########....####..#..##.#.##...#.##..#.###...#..
.###....#.##.#...###...##.#.#.#.#....#...###..#..#.......#....##.....#.#..##.##.#####.##.##.#....#..
#...##.###...#.....#...#..####.#..#..##...###..##..##.....####.#######.#.#.#.##...#...###.##..##.#.#
###...##.###....#..#..###.#....#.#..##..#.#.##....####.###...#####...##...#...#..####.#......##...#.
###....##.#.###...#....###..#######.....##..##.#...####..###.##.#.####..#..##..####...####.#.#.#....
..##...###..##.##.#..###.##..####.#.##.........#####..####..##.##......#...##.#.##.#.#.#.#.#.#####.#
#.#.##.#.#####.##....#.....###....###.....#####.#.###..#.###.##.......#...####...##.#...##.#...##...
..####.###..###..#...###..#.#..##..##..##..##..##..####.########.#.####.....#...#.#.######...#...##.
#.####..#..##.##..#.#.##...#..###.#.....###....#..#.#.......#####.##...##...#...#.....##...##.....##
#.#....#.#####...###.####.#..####.#...##..##..###..##...#..#.#..#.####.#..##.##.#.##.#.#.#.#.####..#
#####.#.#.##.#...#.##.#...#####.#.....#..####...##.###........##....####...#..###..###......#.....##
..#..##..##..##....####.#.#.#...##.###.#...####..#....###.##.#..#.##...#..#.##..#.#.#...#..###....#.
#..#..####.######.##.#.###.#....#.#...##########..##.#....#...#.#.####..##.##..#..##.#...##..####.#.
##.####.####....###..#.###...#.####.#....##.##.#.#####.##.###..###...##.#...###..#.#..###.#.##..#...
#.#..#...##..######.#.####.#.###..##..#...#.#.####.#..##.##....#..###....#...###.#..##.#...#.#.#####
#...#..###...#....#..#.#..##.....#######.#...#...#...###.#.###......######..#.#.###..#.##..###.#....
..###.##..###.......#.##..#.##...##.#.###.##......##...##...##.#.#.##.....##..#.##..#..#..#.#...#..#
.###.####..##..#####..##.###.#...###.#.###.###....#########..#..#.####.##.##.#..##.##...##...#.#.#.#
##...#..#.#.####.#...##....#.####.#....#....###..##.####......###.####...###.###.##..###.#...#...##.
.########....#..##...#.##.#.###.#.#.#..##........#.###..#..####.#....#...####.###.###.####.###.##...
########...#...##.#.####.##..#.###..##..#####.###.###.####..##.##.#.#.#.####.####...##.######.###...
.##...#.##.##.###..####..#####..##.##.####...###..#.######.##.#.#....#.###...#......#.####.###.#..##
..###.#.####.#.#.......#...###.#.####...#..####..#.#######..#..###.#####.#...######.####..#...#..##.
#.#..###...#.#.##....##...#######...#.#..###.#.#...###......#.##.#.#####...##..##.#...###.....#.###.
#...##.#.######..##...##.#..##..#...#.##..#...###..#.##.#..#.....#....###.#...####..#.#...#.###.####
.##....###.#..###.#####.#..#....######.##..##..####..#####..#..###.##.##.#.#.#.###..#.....###...###.
.#.#.###.##.#..#.###.##.##.##..######..#..###...##.#....####.##...#.###.#.#.#.#######..#...#.##.....
.##.#..#####.#...#.#..#.#.#..#.#....#..####...##...###.#...##.#..##.#..##.#.#..#..####...#.####...##
........###...#.#.#.#..#..#.#..#...###..##.#.#..##.##.#..##...#.###.#...#.##..#.###..######..#.#####
.#.#...#.#.###.#..#####.##..#.#...#.#######.#.#.#.#..#.#.#.####.#.##..##.#..#..#..#..##.#####.#....#
#.#.####..####.##...##.####.##.##.#...##..#.#..##.....#####.....#.#...#..#.##..#.#...###...#..###...
.#.#.#..#....###.#.##.#####.###..##.#.##..##.#.####.#.#.#.#.#..##.#.#.#....########..#.###...#.#.#..
###..##....###.##.##...##.#.###.#.#.#.####...##.#.##.#..##..####..#...#.#...###..########.##.###...#
.##.....#......#...####.#....#.#.#.##.....#.#.#.#.##.#.....#.#.....#.###..###.#...##....#.##...#.###
#....##....#.###.###..##...##..#.###.#..#.####...###.#..##....#...#..##.##.##.##.###.#......###.#...
##...#...#.##.#.########.#..#####.###.#..##.#.###..#.###.#..#...#.#.#.#..#...###.#.#....#...####..#.
.###.##.#..#.###.#.#.....#.#......##...#####....#...####...#.###..#..##.#####..#...#..##..#.######..
..####....##.#...###.#....##....####..##.#.##...#...#.....#..####.####...#.#.#####........##.###.###
#..#.##.#..##..#..#....#.#.#.##.#...###...#.....##..###....#..#.#.##.#.#..#.##.#..#......#.........#
.#.......#.####.###...##.########...#..#......##.#..#####..#.##.#..#####.#########.#.##.#.#..#.....#
##.##.....###..#.####.##..#.#.#..#.##..#....#.#.###.###...#####....#.#.#####.##.#....#....#..##....#"#;

        let ans = day_20_part_1(&input, 50);

        assert_eq!(ans, 19228)
    }
}