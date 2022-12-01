use std::convert::TryInto;

pub fn solve_3_part_1(vec: Vec<&str>) -> i32{
    let length = vec[0].len().try_into().unwrap();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for j in 0..length {
        let mut no_1 = 0;
        for i in 0..vec.len(){
            if vec[i].chars().nth(j).unwrap() == '1' {
                no_1+=1;
            }
        }
        if no_1>=vec.len()/2 {
            gamma.push('1');
            epsilon.push('0');
        }
        else{
            gamma.push('0');
            epsilon.push('1');
        }
    }
    i32::from_str_radix(gamma.as_str(), 2).unwrap() * i32::from_str_radix(epsilon.as_str(), 2).unwrap()
}

pub fn solve_3_part_2(vec: Vec<&str>) -> i32{
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