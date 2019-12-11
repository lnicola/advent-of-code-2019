#![allow(dead_code)]

use int_code::{IntCode, State};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Write;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

mod int_code;

fn day1() -> Result<(), Box<dyn Error>> {
    let file = File::open("day1.txt")?;
    let reader = BufReader::new(file);
    let mut fuel1 = 0;
    let mut fuel2 = 0;
    for line in reader.lines() {
        let mut m = line?.parse::<u32>()?;
        m = m / 3 - 2;
        fuel1 += m;
        while m > 6 {
            m = m / 3 - 2;
            fuel2 += m;
        }
    }
    println!("{}", fuel1);
    println!("{}", fuel1 + fuel2);
    Ok(())
}

fn day2() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day2.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()?;

    fn run(mut program: Vec<i128>, noun: i128, verb: i128) -> i128 {
        program[1] = noun;
        program[2] = verb;
        let mut vm = IntCode::from(program);
        vm.run();
        vm.read_mem(0)
    }

    println!("{}", run(program.clone(), 12, 2));

    for noun in 0..100 {
        for verb in 0..100 {
            if run(program.clone(), noun, verb) == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
    Ok(())
}

fn day3() -> Result<(), Box<dyn Error>> {
    let file = File::open("day3.txt")?;
    let reader = BufReader::new(file);
    let mut seen = HashMap::new();
    let mut seen_current = HashSet::new();
    let mut intersections = Vec::new();
    let mut dist_min = usize::max_value();
    let mut delay_min = usize::max_value();

    for line in reader.lines() {
        seen_current.clear();
        let (mut x, mut y) = (0isize, 0isize);
        let mut delay = 0;
        for d in line?.split(',') {
            let dist = d[1..].parse::<usize>()?;
            let (dx, dy) = match &d[0..1] {
                "U" => (0, -1),
                "L" => (-1, 0),
                "D" => (0, 1),
                "R" => (1, 0),
                _ => unreachable!(),
            };
            for _ in 0..dist {
                x += dx;
                y += dy;
                delay += 1;
                if seen_current.insert((x, y)) {
                    if let Some(old_delay) = seen.insert((x, y), delay) {
                        intersections.push((x, y));
                        let d = x.abs() as usize + y.abs() as usize;
                        dist_min = dist_min.min(d);
                        delay_min = delay_min.min(delay + old_delay);
                    }
                }
            }
        }
    }
    println!("{} {}", dist_min, delay_min);
    Ok(())
}

fn day4() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day4.txt")?;
    let pos = input.find('-').expect("invalid input");
    let first = input[..pos].parse::<u32>()?;
    let last = input[pos + 1..].trim_end().parse::<u32>()?;

    let mut count1 = 0;
    let mut count2 = 0;
    for i in first..=last {
        let d1 = i / 100_000;
        let d2 = i / 10_000 % 10;
        let d3 = i / 1_000 % 10;
        let d4 = i / 100 % 10;
        let d5 = i / 10 % 10;
        let d6 = i % 10;
        if (d1 == d2 || d2 == d3 || d3 == d4 || d4 == d5 || d5 == d6)
            && d1 <= d2
            && d2 <= d3
            && d3 <= d4
            && d4 <= d5
            && d5 <= d6
        {
            count1 += 1;
            if d1 == d2 && d2 != d3
                || d2 == d3 && d2 != d1 && d3 != d4
                || d3 == d4 && d3 != d2 && d4 != d5
                || d4 == d5 && d4 != d3 && d5 != d6
                || d5 == d6 && d5 != d4
            {
                count2 += 1;
            }
        }
    }
    println!("{} {}", count1, count2);
    Ok(())
}

fn day5() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day5.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    fn run(program: Vec<i128>, input: i128) -> String {
        let mut output = String::new();
        let mut vm = IntCode::from(program);
        loop {
            match vm.run() {
                State::Output(val) => write!(output, "{}", val).unwrap(),
                State::Input(cookie) => cookie.set(input),
                State::Halted => break,
            }
        }
        output
    }
    println!("{}", run(program.clone(), 1));
    println!("{}", run(program.clone(), 5));

    Ok(())
}

fn day6() -> Result<(), Box<dyn Error>> {
    let file = File::open("day6.txt")?;
    let reader = BufReader::new(file);
    let mut id_map = HashMap::new();
    let mut map = HashMap::new();
    let mut n = 0;
    for line in reader.lines() {
        let line = line?;
        let pos = line.find(')').expect("invalid input");
        let p = line[..pos].to_string();
        let q = line[pos + 1..].trim_end().to_string();
        let p = *id_map.entry(p).or_insert_with(|| {
            n += 1;
            n - 1
        });
        let q = *id_map.entry(q).or_insert_with(|| {
            n += 1;
            n - 1
        });
        map.entry(p).or_insert(vec![]).push(q);
    }
    let com = id_map["COM"];
    let mut checksum = 0;
    let mut stack = Vec::new();
    let mut parent = HashMap::new();
    stack.push((com, 0));
    while let Some((p, d)) = stack.pop() {
        checksum += d;
        if let Some(f) = map.get(&p) {
            for &q in f {
                parent.insert(q, p);
                stack.push((q, d + 1));
            }
        }
    }
    println!("{}", checksum);
    let mut p = id_map["YOU"];
    let mut q = id_map["SAN"];
    let mut seen = HashMap::new();
    let mut d1 = 0;
    let mut d2 = 0;
    let mut d = 0;
    while p != com || q != com {
        if p != com {
            p = parent[&p];
            d1 += 1;
            if let Some(k) = seen.insert(p, d1) {
                d = d1 + k;
                break;
            }
        }
        if q != com {
            q = parent[&q];
            d2 += 1;
            if let Some(k) = seen.insert(q, d2) {
                d = d2 + k;
                break;
            }
        }
    }
    println!("{}", d - 2);
    Ok(())
}

fn day7() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day7.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    let template = IntCode::from(program);
    let mut amps = vec![IntCode::new(); 5];
    let mut max_signal = 0;
    for p1 in 0..5 {
        for p2 in 0..5 {
            if p2 == p1 {
                continue;
            }
            for p3 in 0..5 {
                if p3 == p1 || p3 == p2 {
                    continue;
                }
                for p4 in 0..5 {
                    if p4 == p1 || p4 == p2 || p4 == p3 {
                        continue;
                    }
                    let p5 = 10 - p1 - p2 - p3 - p4;
                    let phase = [p1, p2, p3, p4, p5];
                    for (i, vm) in amps.iter_mut().enumerate() {
                        *vm = template.clone();
                        match vm.run() {
                            State::Input(cookie) => cookie.set(phase[i]),
                            _ => unreachable!(),
                        }
                    }
                    let mut signal = 0;
                    for vm in amps.iter_mut() {
                        match vm.run() {
                            State::Input(cookie) => cookie.set(signal),
                            _ => unreachable!(),
                        }
                        match vm.run() {
                            State::Output(val) => signal = val,
                            _ => unreachable!(),
                        }
                    }
                    max_signal = max_signal.max(signal);
                }
            }
        }
    }
    println!("{}", max_signal);
    max_signal = 0;
    for p1 in 5..10 {
        for p2 in 5..10 {
            if p2 == p1 {
                continue;
            }
            for p3 in 5..10 {
                if p3 == p1 || p3 == p2 {
                    continue;
                }
                for p4 in 5..10 {
                    if p4 == p1 || p4 == p2 || p4 == p3 {
                        continue;
                    }
                    let p5 = 35 - p1 - p2 - p3 - p4;
                    let phase = [p1, p2, p3, p4, p5];
                    for (i, vm) in amps.iter_mut().enumerate() {
                        *vm = template.clone();
                        match vm.run() {
                            State::Input(cookie) => cookie.set(phase[i]),
                            _ => unreachable!(),
                        }
                    }
                    let mut signal = 0;
                    'control: loop {
                        for vm in amps.iter_mut() {
                            match vm.run() {
                                State::Input(cookie) => cookie.set(signal),
                                State::Halted => break 'control,
                                _ => unreachable!(),
                            }
                            match vm.run() {
                                State::Output(val) => signal = val,
                                _ => unreachable!(),
                            }
                        }
                    }
                    max_signal = max_signal.max(signal);
                }
            }
        }
    }
    println!("{}", max_signal);
    Ok(())
}

fn day8() -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string("day8.txt")?
        .trim_end()
        .chars()
        .map(|c| c as u8 - b'0')
        .collect::<Vec<_>>();
    let (width, height) = (25, 6);
    let mut min = i32::max_value();
    let mut r = 0;
    let mut image = vec![0; width * height];
    for layer in buf.as_slice().rchunks(width * height) {
        let mut num0 = 0;
        let mut num1 = 0;
        let mut num2 = 0;
        for (&p, q) in layer.iter().zip(image.iter_mut()) {
            match p {
                0 => {
                    num0 += 1;
                    *q = 0
                }
                1 => {
                    num1 += 1;
                    *q = 1
                }
                2 => num2 += 1,
                _ => {}
            }
        }
        if num0 < min {
            min = num0;
            r = num1 * num2;
        }
    }
    println!("{}", r);
    for line in image.as_slice().chunks(width) {
        for p in line {
            if *p == 0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        println!();
    }
    Ok(())
}

fn day9() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day9.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    fn run(program: Vec<i128>, input: i128) {
        let mut vm = IntCode::from(program);
        loop {
            match vm.run() {
                State::Output(val) => println!("{}", val),
                State::Input(cookie) => cookie.set(input),
                State::Halted => break,
            }
        }
    }
    run(program.clone(), 2);

    Ok(())
}

fn day10() -> Result<(), Box<dyn Error>> {
    let file = File::open("day10.txt")?;
    let reader = BufReader::new(file);

    let mut asteroids = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            match c {
                '#' => asteroids.push((x as f64 + 0.5, y as f64 + 0.5)),
                '.' => {}
                _ => unreachable!(),
            }
        }
    }

    fn dist(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        ((p2.0 - p1.0) * (p2.0 - p1.0) + (p2.1 - p1.1) * (p2.1 - p1.1)).sqrt()
    }

    fn between(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> bool {
        dist(p1, p2) + dist(p2, p3) - dist(p1, p3) < 0.000000001
    }

    let n = asteroids.len();
    let mut asteroids_between = vec![0; n * n];
    let mut best = 0;
    let mut station = 0;
    for (i, &p1) in asteroids.iter().enumerate() {
        let mut score = 0;
        for (j, &p2) in asteroids.iter().enumerate() {
            if j == i {
                continue;
            }
            let mut ok = true;
            for (k, &p3) in asteroids.iter().enumerate() {
                if k != i && k != j && between(p1, p3, p2) {
                    ok = false;
                    asteroids_between[i * n + j] += 1;
                }
            }
            if ok {
                score += 1;
            }
        }
        if score > best {
            best = score;
            station = i;
        }
    }
    println!("{}", best);
    let mut idx = Vec::new();
    let station_p = asteroids[station];
    let mut angle_map = HashMap::new();
    for (i, p) in asteroids.iter().enumerate() {
        if i != station {
            let dx = p.0 - station_p.0;
            let dy = p.1 - station_p.1;
            let angle = dx.atan2(-dy);
            let t = (angle * 1_000_000.0) as u32;
            angle_map.entry(t).or_insert(vec![]).push(i);
        }
    }
    let mut angle_map = angle_map.into_iter().collect::<Vec<_>>();
    angle_map.sort_by_key(|r| r.0);
    let mut by_angle = Vec::new();
    for (_, mut points) in angle_map {
        points.sort_by_key(|i| -asteroids_between[station * n + i]);
        by_angle.push(points);
    }
    while idx.len() < asteroids.len() - 1 {
        for ray in &mut by_angle {
            if let Some(p) = ray.pop() {
                idx.push(p);
            }
        }
    }

    println!(
        "{}",
        asteroids[idx[199]].0 as usize * 100 + asteroids[idx[199]].1 as usize
    );
    Ok(())
}

fn day11() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day11.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    let vm = IntCode::from(program);
    let mut wall = HashMap::new();
    fn run(mut vm: IntCode, wall: &mut HashMap<(i128, i128), bool>) {
        let (mut x, mut y, mut dir) = (0, 0, 0);
        const D: [(i128, i128); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
        loop {
            match vm.run() {
                State::Input(cookie) => cookie.set(*wall.get(&(x, y)).unwrap_or(&false) as i128),
                State::Output(val) => {
                    let val = match val {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    };
                    wall.insert((x, y), val);
                    match vm.run() {
                        State::Output(0) => dir = (dir + 3) % 4,
                        State::Output(1) => dir = (dir + 1) % 4,
                        _ => unreachable!(),
                    }
                    x += D[dir].0;
                    y += D[dir].1;
                }
                State::Halted => break,
            }
        }
    }
    run(vm.clone(), &mut wall);
    println!("{}", wall.len());
    wall.clear();
    wall.insert((0, 0), true);
    run(vm, &mut wall);
    let mut minx = i128::max_value();
    let mut maxx = i128::min_value();
    let mut miny = i128::max_value();
    let mut maxy = i128::min_value();
    for &(x, y) in wall.keys() {
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);
    }
    for y in miny..=maxy {
        for x in minx..=maxx {
            match *wall.get(&(maxx + minx - x, y)).unwrap_or(&false) {
                false => print!(" "),
                true => print!("█"),
            }
        }
        println!();
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    day11()
}
