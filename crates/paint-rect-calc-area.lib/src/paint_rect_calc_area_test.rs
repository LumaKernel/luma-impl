use crate::paint_rect;
use rand::{self, Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[test]
fn test_simple_0() {
    // rect[0]
    // 11.
    // 11.
    //
    // rect[1]
    // .11
    // .11
    //
    // all
    // 121
    // 121
    assert_eq!(
        paint_rect().add(0, 0, 2, 2).add(1, 0, 3, 2).calc_area(),
        6_u32
    );
}

#[test]
fn test_simple_1() {
    assert_eq!(paint_rect().add(0, 0, 1, 1).calc_area(), 1_u32);
    assert_eq!(paint_rect().add(0, 0, 2, 2).calc_area(), 4_u32);
    assert_eq!(
        paint_rect().add(0, 0, 1000000, 1000).calc_area(),
        1000000000_u32
    );
}

#[test]
fn test_corner_len0() {
    assert_eq!(paint_rect::<i32>().calc_area(), 0_u32);
}

#[test]
fn test_corner_area0() {
    assert_eq!(paint_rect().add(0, 0, 0, 0).calc_area(), 0_u32);
    assert_eq!(paint_rect().add(0, 0, 1, 0).calc_area(), 0_u32);
    assert_eq!(paint_rect().add(0, 0, 0, 1).calc_area(), 0_u32);
    assert_eq!(paint_rect().add(0, 0, u32::MAX, 0).calc_area(), 0_u32);
    assert_eq!(paint_rect().add(0, 0, 0, u32::MAX).calc_area(), 0_u32);
}

#[test]
fn test_corner_max() {
    assert_eq!(paint_rect().add(0, 0, u8::MAX, 1).calc_area(), u8::MAX);
    assert_eq!(paint_rect().add(0, 0, u16::MAX, 1).calc_area(), u16::MAX);
    assert_eq!(paint_rect().add(0, 0, u32::MAX, 1).calc_area(), u32::MAX);
    assert_eq!(paint_rect().add(0, 0, u64::MAX, 1).calc_area(), u64::MAX);
    assert_eq!(paint_rect().add(0, 0, u128::MAX, 1).calc_area(), u128::MAX);

    assert_eq!(paint_rect().add(0, 0, 1, u8::MAX).calc_area(), u8::MAX);
    assert_eq!(paint_rect().add(0, 0, 1, u16::MAX).calc_area(), u16::MAX);
    assert_eq!(paint_rect().add(0, 0, 1, u32::MAX).calc_area(), u32::MAX);
    assert_eq!(paint_rect().add(0, 0, 1, u64::MAX).calc_area(), u64::MAX);
    assert_eq!(paint_rect().add(0, 0, 1, u128::MAX).calc_area(), u128::MAX);
}

#[test]
fn test_corner_max32() {
    assert_eq!(
        paint_rect()
            .add(
                i32::MIN as i64,
                i32::MIN as i64,
                i32::MAX as i64,
                i32::MAX as i64,
            )
            .calc_area(),
        (u32::MAX as u64).pow(2),
    );
}

#[test]
fn test_small_0() {
    // rect[0]
    // .111...
    // .111...
    // .111...
    // .......
    //
    // rect[1]
    // .......
    // ...111.
    // ...111.
    // .......
    //
    // rect[2]
    // .......
    // .......
    // .......
    // ......1
    //
    // rect[3]
    // .......
    // .....1.
    // .....1.
    // .......
    //
    // rect[4]
    // .......
    // ..1....
    // .......
    // .......
    //
    // all
    // .111...
    // .12212.
    // .11212.
    // ......1
    assert_eq!(
        paint_rect()
            .add(1, 0, 4, 3)
            .add(3, 1, 6, 3)
            .add(6, 3, 7, 4)
            .add(2, 1, 3, 2)
            .calc_area(),
        14_u32
    );
}

#[test]
fn test_small_1() {
    // rect[0]
    // .1111111...
    // .1111111...
    // .1111111...
    // ...........
    //
    // rect[1]
    // ...........
    // ...........
    // ...11111111
    // ...11111111
    //
    // all
    // .1111111...
    // .1111111...
    // .1122222111
    // ...11111111
    assert_eq!(
        paint_rect()
            .add(1_u16, 0, 8, 3)
            .add(3, 2, 11, 4)
            .calc_area(),
        32
    );
}

#[test]
fn test_large() {
    assert_eq!(
        paint_rect::<i64>()
            .add(-185829, 284, 4785690, 9218)
            .add(459278, -592, 2848589, 3)
            .add(58895, 28684, 176790, 1333351)
            .add(-459, -1833, 5849028, 288456)
            .calc_area(),
        1822885723348,
    );
}

#[test]
fn test_full() {
    let mut rng = Xoshiro256PlusPlus::from_seed([0; 32]);
    for (n, k) in [
        (3, 1000),
        (10, 300),
        (50, 100),
        (100, 5),
        (100, 15),
        (100, 100),
    ] {
        for _ in 0..3 {
            let mut board = vec![vec![false; n]; n];
            let mut p = paint_rect::<i64>();
            for _ in 0..k {
                let x1 = rng.gen_range(0..n);
                let y1 = rng.gen_range(0..n);
                let x2 = rng.gen_range(x1..=n);
                let y2 = rng.gen_range(y1..=n);
                for row in board.iter_mut().take(y2).skip(y1) {
                    for cell in row.iter_mut().take(x2).skip(x1) {
                        *cell = true;
                    }
                }
                p = p.add(x1 as i64, y1 as i64, x2 as i64, y2 as i64);
            }
            let expected = board.iter().flatten().filter(|&&b| b).count() as u64;
            println!("n = {}, k = {}", n, k);
            for row in board.iter() {
                for &cell in row {
                    print!("{}", if cell { '#' } else { '.' });
                }
                println!();
            }
            assert_eq!(p.calc_area(), expected);
        }
    }
}
