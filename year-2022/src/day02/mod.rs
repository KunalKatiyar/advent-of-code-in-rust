pub fn solve_2_part_1(vec: Vec<&str>) -> i32{
    let mut x=0;
    let mut y=0;
    for ind in 0..vec.len(){
        if vec[ind].split_whitespace().collect::<Vec<&str>>()[0] == "forward" {
            x+=vec[ind].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
        else if vec[ind].split_whitespace().collect::<Vec<&str>>()[0] == "up" {
            y-=vec[ind].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
        else if vec[ind].split_whitespace().collect::<Vec<&str>>()[0] == "down" {
            y+=vec[ind].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
    }
    x*y
}

pub fn solve_2_part_2(vec: Vec<&str>) -> i32{
    let mut x=0;
    let mut y=0;
    let mut aim=0;
    for ind in 0..vec.len(){
        if vec[ind].split_whitespace().collect::<Vec<&str>>()[0] == "forward" {
            x+=vec[ind].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            y+=vec[ind].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap()*aim;
        }
        else if vec[ind].split_whitespace().collect::<Vec<&str>>()[0] == "up" {
            aim-=vec[ind].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
        else if vec[ind].split_whitespace().collect::<Vec<&str>>()[0] == "down" {
            aim+=vec[ind].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
    }
    x*y
}