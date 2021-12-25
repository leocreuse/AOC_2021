fn main() {
    let mut count = 0;
    let xmin = 137;
    let xmax = 171;
    let ymin = -73;
    let ymax = -98;
    for vx in 0 .. xmax + 1 {
        for vy in ymax .. -ymax + 1 {
            let mut x = 0;
            let mut cur_vx = vx;
            let mut y = 0;
            let mut cur_vy = vy;
            while x <= xmax && y >= ymax {
                x += cur_vx;
                cur_vx = if cur_vx == 0 {0} else {cur_vx - 1};
                y += cur_vy;
                cur_vy -= 1;
                if x <= xmax && y >= ymax && x >= xmin && y <= ymin {
                    count +=1;
                    break;
                }
            }
        }
    }
    println!("{}", count)
}
