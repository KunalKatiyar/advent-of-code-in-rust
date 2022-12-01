use std::convert::TryInto;

pub fn solve_4_part_1(vec: Vec<&str>) -> i32{
    let seq = vec[0].split(",").collect::<Vec<&str>>();
    let ans=0;
    ans
}

pub fn solve_4_part_2(vec: Vec<&str>) -> i32{
    let mut vec1 = vec.clone();
    let length = vec1[0].len().try_into().unwrap();
    for j in 0..length {
        let mut no_1 = 0;
        for i in 0..vec1.len(){
            if vec1[i].chars().nth(j).unwrap() == '1' {
                no_1+=1;
            }
        }
        if no_1>=vec1.len()/2 {
            vec1.retain(|&x| x.chars().nth(j).unwrap() == '1');
        }
        else{
            vec1.retain(|&x| x.chars().nth(j).unwrap() == '0');
        }
        if vec1.len()==1 {
            break;
        }
    }
    let mut vec2 = vec.clone();
    let length = vec2[0].len().try_into().unwrap();
    for j in 0..length {
        let mut no_1 = 0;
        for i in 0..vec2.len(){
            if vec2[i].chars().nth(j).unwrap() == '1' {
                no_1+=1;
            }
        }
        let no_0 = vec2.len()-no_1;
        if no_0<=vec2.len()/2 {
            vec2.retain(|&x| x.chars().nth(j).unwrap() == '0');
        }
        else{
            vec2.retain(|&x| x.chars().nth(j).unwrap() == '1');
        }
        if vec2.len()==1 {
            break;
        }
    }
    i32::from_str_radix(vec1[0], 2).unwrap() * i32::from_str_radix(vec2[0], 2).unwrap()
}