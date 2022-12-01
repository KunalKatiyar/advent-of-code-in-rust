use std::cmp;

pub fn solve_1_part_1(vec: Vec<&str>) -> i32{
    let mut mid_ans = 0;
    let mut vec1 = Vec::new();
    for ind in 1..vec.len(){
        match vec[ind].parse::<i32>() {
            Ok(x)=>{
                mid_ans+=x;
            },
            Err(_)=>{vec1.push(mid_ans);mid_ans=0;continue;}
        }
        // if vec[ind].parse::<i32>().unwrap() > vec[ind-1].parse::<i32>().unwrap() {
        //     ans+=1;
        // }
    }
    vec1.sort();
    vec1[vec1.len()-1]+vec1[vec1.len()-2]+vec1[vec1.len()-3]
}

pub fn solve_1_part_2(vec: Vec<&str>) -> i32{
    let mut ans = 0;
    for ind in 3..vec.len(){
        if vec[ind-3].parse::<i32>().unwrap() < vec[ind].parse::<i32>().unwrap() {
            ans+=1;
        }
    }
    ans
}