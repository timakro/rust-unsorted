use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

const MONSTER: [&[u8; 20]; 3] = [
    b"                  # ",
    b"#    ##    ##    ###",
    b" #  #  #  #  #  #   ",
];
const MONSTER_H: usize = MONSTER.len();
const MONSTER_W: usize = MONSTER[0].len();

#[derive(Debug, Copy, Clone)]
struct Tile {
    id: u32,
    borders: [u16; 8],
    inner: [[bool; 8]; 8],
}

impl Tile {
    fn rotate(&self) -> Tile {
        let b = self.borders;
        let mut inner = [[false; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                inner[i][j] = self.inner[j][7-i];
            }
        }
        Tile {
            id: self.id,
            borders: [b[2], b[3], b[4], b[5], b[6], b[7], b[0], b[1]],
            inner,
        }
    }

    fn flip(&self) -> Tile {
        let b = self.borders;
        let mut inner = [[false; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                inner[i][j] = self.inner[i][7-j];
            }
        }
        Tile {
            id: self.id,
            borders: [b[1], b[0], b[7], b[6], b[5], b[4], b[3], b[2]],
            inner,
        }
    }
}

fn int_sqrt(x: i32) -> Option<i32> {
    for sr in 0..x {
        if sr * sr == x {
            return Some(sr);
        }
    }
    None
}

fn find_monster(image: &HashMap<(i32,i32), Tile>, size: i32) {
    let get = |i: usize, j: usize| -> bool {
        image.get(&(j as i32 / 8,i as i32 / 8)).unwrap().inner[i%8][j%8]
    };

    let mut monster_count = 0;
    let size = (size * 8) as usize;
    for i in 0..=size-MONSTER_H {
        for j in 0..=size-MONSTER_W {
            let mut found = true;
            'monster: for di in 0..MONSTER_H {
                for dj in 0..MONSTER_W {
                    if MONSTER[di][dj] == b'#' && !get(i+di, j+dj) {
                        found = false;
                        break 'monster;
                    }
                }
            }
            if found {
                monster_count += 1;
            }
        }
    }

    if monster_count > 0 {
        println!("Found {} monsters", monster_count);
        let mut roughness = 0;
        for i in 0..size {
            for j in 0..size {
                if get(i,j) { roughness += 1 }
            }
        }
        for i in 0..MONSTER_H {
            for j in 0..MONSTER_W {
                if MONSTER[i][j] == b'#' { roughness -= monster_count }
            }
        }
        println!("Water roughness is {}", roughness);
    }
}

fn place_tile(image: &mut HashMap<(i32,i32), Tile>,
              tiles: &Vec<Tile>,
              conn_left:     &HashMap<u16,       Vec<Tile>>,
              conn_top:      &HashMap<u16,       Vec<Tile>>,
              conn_left_top: &HashMap<(u16,u16), Vec<Tile>>,
              used: HashSet<u32>, n: i32, size: i32) {
    if n == size * size {
        let mut corner_prod: u64 = 1;
        println!("Found solution with corner tiles:");
        for &x in &[0, size-1] {
            for &y in &[0, size-1] {
                let corner = image.get(&(x,y)).unwrap().id;
                corner_prod *= corner as u64;
                print!("{} ", corner);
            }
            println!("");
        }
        println!("The IDs multiply to {}", corner_prod);
        find_monster(image, size);
        println!("");
        return;
    }
    let (x, y) = (n % size, n / size);

    let right   = image.get(&(x-1,y)).map(|x| x.borders[3]);
    let bottom  = image.get(&(x,y-1)).map(|x| x.borders[5]);
    let conn_tiles: Option<&Vec<Tile>> = match (right, bottom) {
        (Some(right), Some(bottom)) => conn_left_top.get(&(right,bottom)),
        (Some(right), None) => conn_left.get(&right),
        (None, Some(bottom)) => conn_top.get(&bottom),
        (None, None) => Some(&tiles),
    };

    if let Some(conn_tiles) = conn_tiles {
        for &tile in conn_tiles {
            if used.contains(&tile.id) {
                continue;
            }
            let mut used = used.clone();
            used.insert(tile.id);

            image.insert((x,y), tile);
            place_tile(image, tiles, conn_left, conn_top, conn_left_top,
                       used, n + 1, size);
        }
    }
}

fn main() {
    let mut tiles: Vec<Tile> = Vec::new();
    let mut conn_left:     HashMap<u16,       Vec<Tile>> = HashMap::new();
    let mut conn_top:      HashMap<u16,       Vec<Tile>> = HashMap::new();
    let mut conn_left_top: HashMap<(u16,u16), Vec<Tile>> = HashMap::new();
    let mut tile_count = 0;

    for block in fs::read_to_string("input").unwrap().split_terminator("\n\n") {
        let mut line_it = block.lines();
        let id = line_it.next().unwrap();
        let id = id[5..id.len()-1].parse().unwrap();
        let mut tile:  [[bool; 10]; 10] = [[false; 10]; 10];
        let mut inner: [[bool;  8];  8] = [[false;  8];  8];
        for (i, line) in line_it.enumerate() {
            for (j, chr) in line.chars().enumerate() {
                let wavy = chr == '#';
                tile[i][j] = wavy;
                if 1 <= i && i <= 8 && 1 <= j && j <= 8 {
                    inner[i-1][j-1] = wavy;
                }
            }
        }

        let mut borders: [u16; 8] = [0; 8];
        let (mut sx, mut sy) = (0, 0);
        let mut i = 0;
        for &(mut dx, mut dy) in &[(1,0), (0,1), (-1,0), (0,-1)] {
            for &reverse in &[false, true] {
                if reverse {
                    sx += dx; sy += dy;
                    dx = -dx; dy = -dy;
                }
                let mut bit = 1 << 9;
                let (mut x, mut y) = (sx * 9, sy * 9);
                for _ in 0..10 {
                    if tile[y as usize][x as usize] {
                        borders[i] += bit;
                    }
                    x += dx; y += dy;
                    bit >>= 1;
                }
                i += 1;
            }
        }

        let mut tile = Tile { id, borders, inner };
        for _ in 0..2 {
            for _ in 0..4 {
                let left = tile.borders[6];
                let top  = tile.borders[0];
                tiles.push(tile);
                conn_left.entry(left).or_default().push(tile);
                conn_top.entry(top).or_default().push(tile);
                conn_left_top.entry((left,top)).or_default().push(tile);
                tile = tile.rotate();
            }
            tile = tile.flip();
        }
        tile_count += 1;
    }

    let mut image: HashMap<(i32,i32), Tile> = HashMap::new();
    let size = int_sqrt(tile_count).unwrap();
    place_tile(&mut image, &tiles, &conn_left, &conn_top, &conn_left_top,
               HashSet::new(), 0, size);
}
