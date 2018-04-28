use std::io::stdin;

fn read_number_vec() -> Vec<usize> {
    let mut a_str = String::new();
    stdin().read_line(&mut a_str).expect("read error");
    a_str.split_whitespace()
        .map(|x| x.parse::<usize>().expect("parse error"))
        .collect::<Vec<usize>>()
}

fn read_number() -> usize {
    let mut s=String::new();
    let mut val:usize = 0;
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let trimmed = s.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => val = i,
        Err(..) => println!("Error"),
    };
    val
}

fn create_neighour_matrix(nodes:usize, connections:usize) -> Vec<Vec<usize>> {
    let mut group:Vec<Vec<usize>> = vec![Vec::new(); nodes];
    // let mut group:Vec<Vec<usize>> = Vec::with_capacity(nodes);
    // for _ in 0..nodes {
    //     let mut row:Vec<usize> = Vec::new();
    //     group.push(row);
    // }
    for _ in 0..connections {
        let uint_vec = read_number_vec();
        group[uint_vec[0]].push(uint_vec[1]);
        group[uint_vec[1]].push(uint_vec[0]);
        // v.push((uint_vec[0], uint_vec[1]));
    }
    group
}

fn solve() {
    let nodes = read_number();
    let connections = read_number();
    let group:Vec<Vec<usize>> = create_neighour_matrix(nodes, connections);

    let mut v: Vec<usize> = Vec::new();
    let mut visited = vec![false; nodes];
    let mut count = 0;
    for i in 0..nodes {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        count += 1;
        v.push(i);
        while let Some(j) = v.pop() {
            for &k in group[j].iter() {
                if !visited[k] {
                    visited[k] = true;
                    v.push(k)
                }
            }

        }
    }
    println!("{}", count - 1);
}

fn main() {
    let testcases = read_number();
    for _ in 0..testcases {
        solve();
    }
}
