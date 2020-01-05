static dist: [[i32; 2]; 8] = [
    [-1, 0], [-1, -1], [0, -1], [1, -1],
    [1, 0], [1, 1], [0, 1], [-1, 1]
];

pub fn new(sz:usize) -> Vec<Vec<bool>> {
  vec![vec![false; sz]; sz]
}

pub fn life(sz:usize,stge:&Vec<Vec<bool>>) -> Vec<Vec<bool>>{
  let mut nextstge = new(sz);
  for x in 0..sz {
    for y in 0..sz{
      //4つ調べる
      let mut adj = 0;
      for pos in dist.iter() {
        let _x = x as i32 + pos[0];
        let _y = y as i32 + pos[1];
        if 0 <= _x && _x < sz as i32 -1 && 0 <= _y && _y <= sz as i32 -1 && stge[_x as usize][_y as usize] {
          adj+=1;
        }
        
      }
      if stge[x][y] {
        if adj == 2 || adj == 3 {
          nextstge[x][y] = true;
        } else{
          nextstge[x][y] = false;
        }
      } else {
        if adj == 3 {
          nextstge[x][y] = true;
        }
      }
    }
  }
  nextstge
}