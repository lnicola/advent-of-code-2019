#![allow(dead_code)]

use int_code::{IntCode, State};
use interner::Interner;
use num::Integer;
use priority_queue::PriorityQueue;
use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt::Write;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::iter;
use std::ops::{Mul, MulAssign};

mod int_code;
mod interner;
mod iterator;

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
            if run(program.clone(), noun, verb) == 19_690_720 {
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
    println!("{}", run(program, 5));

    Ok(())
}

fn day6() -> Result<(), Box<dyn Error>> {
    let file = File::open("day6.txt")?;
    let reader = BufReader::new(file);
    let mut objects = Interner::new();
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let pos = line.find(')').expect("invalid input");
        let p = line[..pos].to_string();
        let q = line[pos + 1..].trim_end().to_string();
        let p = objects.insert(p);
        let q = objects.insert(q);
        map.entry(p).or_insert_with(Vec::new).push(q);
    }
    let com = objects["COM"];
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
    print!("{} ", checksum);

    let mut p = objects["YOU"];
    let mut q = objects["SAN"];
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

    run(program.clone(), 1);
    run(program, 2);

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
        dist(p1, p2) + dist(p2, p3) - dist(p1, p3) < 0.000_000_001
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
            angle_map.entry(t).or_insert_with(Vec::new).push(i);
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
            if *wall.get(&(maxx + minx - x, y)).unwrap_or(&false) {
                print!("█");
            } else {
                print!(" ")
            }
        }
        println!();
    }
    Ok(())
}

fn day12() -> Result<(), Box<dyn Error>> {
    let file = File::open("day12.txt")?;
    let reader = BufReader::new(file);
    let mut px = Vec::new();
    let mut py = Vec::new();
    let mut pz = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(['=', ',', '>'].as_ref());
        parts.next();
        let x = parts.next().unwrap().parse::<i32>()?;
        parts.next();
        let y = parts.next().unwrap().parse::<i32>()?;
        parts.next();
        let z = parts.next().unwrap().parse::<i32>()?;
        px.push(x);
        py.push(y);
        pz.push(z);
    }
    let mut vx = vec![0; px.len()];
    let mut vy = vec![0; py.len()];
    let mut vz = vec![0; pz.len()];
    let (mut states_x, mut states_y, mut states_z) = Default::default();
    let (mut per_x, mut per_y, mut per_z) = (None::<u64>, None::<u64>, None::<u64>);
    let mut time = 0u64;

    fn insert_state(states: &mut HashSet<Box<[i32]>>, p: &[i32], v: &[i32]) -> bool {
        states.insert(
            ichain!(p.iter().copied(), v.iter().copied())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    loop {
        if let (Some(per_x), Some(per_y), Some(per_z)) = (per_x, per_y, per_z) {
            let period = per_x.lcm(&per_y).lcm(&per_z);
            println!("{}", period);
            break;
        }
        if per_x.is_none() && !insert_state(&mut states_x, px.as_slice(), vx.as_slice()) {
            per_x = Some(time);
        }
        if per_y.is_none() && !insert_state(&mut states_y, py.as_slice(), vy.as_slice()) {
            per_y = Some(time);
        }
        if per_z.is_none() && !insert_state(&mut states_z, pz.as_slice(), vz.as_slice()) {
            per_z = Some(time);
        }
        time += 1;

        fn update_velocities(p: &[i32], v: &mut [i32]) {
            assert_eq!(p.len(), v.len());
            for i in 0..p.len() {
                for j in i + 1..p.len() {
                    match p[i].cmp(&p[j]) {
                        Ordering::Less => {
                            v[i] += 1;
                            v[j] -= 1;
                        }
                        Ordering::Greater => {
                            v[i] -= 1;
                            v[j] += 1;
                        }
                        _ => {}
                    }
                }
            }
        }

        update_velocities(px.as_slice(), vx.as_mut_slice());
        update_velocities(py.as_slice(), vy.as_mut_slice());
        update_velocities(pz.as_slice(), vz.as_mut_slice());

        for (p, v) in ichain!(
            izip!(&mut px, &vx),
            izip!(&mut py, &vy),
            izip!(&mut pz, &vz)
        ) {
            *p += *v;
        }
        if time == 1000 {
            let mut e = 0;
            for (px, py, pz, vx, vy, vz) in izip!(&px, &py, &pz, &vx, &vy, &vz) {
                let pot = px.abs() + py.abs() + pz.abs();
                let kin = vx.abs() + vy.abs() + vz.abs();
                e += pot * kin;
            }
            print!("{} ", e);
        }
    }
    Ok(())
}

fn day13() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day13.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut vm = IntCode::from(program);
    let mut ball_x = None::<i128>;
    let mut paddle_x = None;
    let mut score = 0;
    let mut blocks = None;

    vm.write_mem(0, 2);
    loop {
        let x = match vm.run() {
            State::Output(val) => val,
            State::Halted => {
                break;
            }
            State::Input(cookie) => {
                if let Some(blocks) = blocks.take() {
                    print!("{} ", blocks);
                }
                match (ball_x, paddle_x) {
                    (Some(ball_x), Some(paddle_x)) => cookie.set(ball_x.cmp(&paddle_x) as i128),
                    _ => unreachable!(),
                }
                continue;
            }
        };
        let y = match vm.run() {
            State::Output(val) => val,
            _ => unreachable!(),
        };
        match vm.run() {
            State::Output(val) if x == -1 && y == 0 => score = val,
            State::Output(0) | State::Output(1) => {}
            State::Output(2) => *blocks.get_or_insert(0) += 1,
            State::Output(3) => paddle_x = Some(x),
            State::Output(4) => ball_x = Some(x),
            _ => unreachable!(),
        };
    }
    println!("{}", score);
    Ok(())
}

fn day14() -> Result<(), Box<dyn Error>> {
    let file = File::open("day14.txt")?;
    let reader = BufReader::new(file);

    #[derive(Clone, Debug)]
    struct MaterialQuantity {
        material: usize,
        quantity: i64,
    }

    #[derive(Clone, Debug)]
    struct Reaction {
        inputs: Vec<MaterialQuantity>,
        output: MaterialQuantity,
    }

    let mut materials = Interner::new();
    let mut reactions = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split([' ', ',', '=', '>'].as_ref()).rev();

        let output = MaterialQuantity {
            material: materials.insert(parts.next().unwrap().to_string()),
            quantity: parts.next().unwrap().parse()?,
        };
        parts.next();
        parts.next();
        parts.next();

        let mut inputs = Vec::new();
        loop {
            inputs.push(MaterialQuantity {
                material: materials.insert(parts.next().unwrap().to_string()),
                quantity: parts.next().unwrap().parse()?,
            });
            if parts.next().is_none() {
                break;
            }
        }
        let reaction = Reaction { inputs, output };
        reactions.insert(reaction.output.material, reaction);
    }
    let ore = materials["ORE"];
    let fuel = materials["FUEL"];

    fn run(
        reactions: &HashMap<usize, Reaction>,
        fuel_goal: i64,
        fuel_id: usize,
        ore_id: usize,
    ) -> i64 {
        let fuel = MaterialQuantity {
            material: fuel_id,
            quantity: fuel_goal,
        };
        let mut goals = vec![fuel];
        let mut available = HashMap::new();
        let mut ore = 0;
        while let Some(mut goal) = goals.pop() {
            if goal.material == ore_id {
                ore += goal.quantity;
                continue;
            }
            let have = available.entry(goal.material).or_insert(0);
            let m = goal.quantity.min(*have);
            goal.quantity -= m;
            *have -= m;
            if goal.quantity > 0 {
                let reaction = &reactions[&goal.material];
                let k = (goal.quantity + reaction.output.quantity - 1) / reaction.output.quantity;
                for r in &reaction.inputs {
                    let mut want = r.quantity * k;
                    let have = available.entry(r.material).or_insert(0);
                    let m = want.min(*have);
                    want -= m;
                    *have -= m;
                    goals.push(MaterialQuantity {
                        material: r.material,
                        quantity: want,
                    });
                }
                let d = reaction.output.quantity * k - goal.quantity;
                *available.entry(goal.material).or_insert(0) += d;
            }
        }
        ore
    }

    let ore_available = 1_000_000_000_000i64;
    let r = run(&reactions, 1, fuel, ore);
    let mut lo = ore_available / r;
    let mut d = 1;
    while run(&reactions, lo + d, fuel, ore) < ore_available {
        d *= 2;
    }
    let mut hi = lo + d;
    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let needed = run(&reactions, m, fuel, ore);
        if needed <= ore_available {
            lo = m;
        } else {
            hi = m - 1;
        }
    }
    println!("{} {}", r, lo);
    Ok(())
}

fn day15() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day15.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut vm = IntCode::from(program);
    let mut map = HashMap::new();

    let (mut x, mut y) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    map.insert((0, 0), (false, 5));

    const D: [(i128, i128); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    'main: loop {
        for k in 0..4u8 {
            let (nx, ny) = (x + D[k as usize].0, y + D[k as usize].1);
            if map.get(&(nx, ny)).is_some() {
                continue;
            }

            match vm.run() {
                State::Input(cookie) => {
                    cookie.set((k + 1) as i128);
                }
                _ => unreachable!(),
            }
            match vm.run() {
                State::Output(0) => {
                    map.insert((nx, ny), (true, 5));
                }
                State::Output(t) => {
                    if t == 2 {
                        tx = nx;
                        ty = ny;
                    } else if t != 1 {
                        unreachable!()
                    }
                    x = nx;
                    y = ny;
                    let prev_direction = match k {
                        0 => 1,
                        1 => 0,
                        2 => 3,
                        3 => 2,
                        _ => unreachable!(),
                    };
                    map.insert((nx, ny), (false, prev_direction));
                    continue 'main;
                }
                _ => unreachable!(),
            }
        }
        match map.get(&(x, y)) {
            Some((_, 5)) => break,
            Some((_, prev_direction)) => {
                match vm.run() {
                    State::Input(cookie) => {
                        cookie.set((prev_direction + 1) as i128);
                    }
                    _ => unreachable!(),
                }
                match vm.run() {
                    State::Output(1) => {
                        x += D[*prev_direction as usize].0;
                        y += D[*prev_direction as usize].1;
                        continue 'main;
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    let mut stack = vec![(tx, ty, 0)];
    let mut visited = HashMap::new();
    while let Some((x, y, d)) = stack.pop() {
        if x == 0 && y == 0 {
            print!("{} ", d);
        }
        visited.insert((x, y), d);
        for k in 0..4 {
            let (nx, ny) = (x + D[k].0, y + D[k].1);
            if let Some((true, _)) = map.get(&(nx, ny)) {
                continue;
            }
            match visited.get(&(nx, ny)) {
                Some(prev_d) if *prev_d < d + 1 => {
                    continue;
                }
                _ => {
                    visited.insert((nx, ny), d + 1);
                    stack.push((nx, ny, d + 1));
                }
            }
        }
    }
    println!("{}", visited.values().max().unwrap_or(&0));
    Ok(())
}

fn day16() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day16.txt")?;
    let mut input = input
        .trim()
        .chars()
        .map(|c| (c as u8 - b'0') as i32)
        .collect::<Vec<_>>();

    let mut next = vec![0; input.len()];

    let mut offset = 0;
    for i in 0..7 {
        offset = offset * 10 + input[i];
    }

    let mut suffix = input
        .iter()
        .copied()
        .rev()
        .cycle()
        .take(input.len() * 10_000 - offset as usize)
        .collect::<Vec<_>>();

    for _ in 0..100 {
        for i in 0..input.len() {
            let pattern = iter::repeat(0)
                .take(i + 1)
                .chain(iter::repeat(1).take(i + 1))
                .chain(iter::repeat(0).take(i + 1))
                .chain(iter::repeat(-1).take(i + 1))
                .cycle()
                .skip(1);
            next[i] = input
                .iter()
                .zip(pattern)
                .map(|(i, p)| i * p)
                .sum::<i32>()
                .abs()
                % 10;
        }
        input.copy_from_slice(&next);
    }
    for i in 0..8 {
        print!("{}", input[i]);
    }
    print!(" ");

    for _ in 0..100 {
        let mut sum = 0;
        for d in &mut suffix {
            sum += *d;
            *d = sum.abs() % 10;
        }
    }
    for d in suffix.iter().rev().take(8) {
        print!("{}", d);
    }
    println!();
    Ok(())
}

fn day17() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day17.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut vm = IntCode::from(program);
    let mut map = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut cnt = 0;
    let mut alignment = 0;
    vm.write_mem(0, 2);
    let program = r"A,B,A,C,A,B,C,B,C,A
L,12,R,4,R,4,L,6
L,12,R,4,R,4,R,12
L,10,L,6,R,4
n
";
    let mut program_it = program.chars();
    loop {
        match vm.run() {
            State::Output(val) => {
                let c = val as u8 as char;
                if height == 0 {
                    if width == 0 && c == '\n' {
                        width = cnt;
                    } else {
                        cnt += 1;
                        if c != '\n' {
                            map.push(c);
                        }
                    }
                }
                if val > 0x7f {
                    println!("{}", val);
                }
            }
            State::Input(cookie) => {
                if height == 0 {
                    height = map.len() / width;
                    for y in 1..height - 1 {
                        for x in 1..width - 1 {
                            if map[y * width + x] == '#'
                                && map[y * width + x - 1] == '#'
                                && map[y * width + x + 1] == '#'
                                && map[(y - 1) * width + x] == '#'
                                && map[(y + 1) * width + x] == '#'
                            {
                                alignment += y * x;
                                map[y * width + x] = 'O';
                            }
                        }
                    }
                    println!("{}", alignment);
                }
                match program_it.next() {
                    Some(c) => {
                        cookie.set(c as i128);
                    }
                    None => unreachable!(),
                }
            }
            State::Halted => break,
        }
    }
    Ok(())
}

fn day18() -> Result<(), Box<dyn Error>> {
    let file = File::open("day18.txt")?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    let mut width = 0;
    let (mut x_start, mut y_start) = (0, 0);
    let mut num_keys = 0;
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                x_start = x;
                y_start = y;
                map.push('@');
            } else {
                map.push(c);

                if ('a'..='z').contains(&c) {
                    num_keys += 1;
                }
            }
        }
    }

    let height = map.len() / width;

    const D: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    let mut frontier = PriorityQueue::new();
    frontier.push((0, x_start, y_start), Reverse(0));
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut dist = vec![i16::max_value(); width * height];
    while let Some(((keys, x, y), d1)) = frontier.pop() {
        if keys == (1 << num_keys) - 1 {
            print!("{} ", d1.0);
            break;
        }
        visited.insert((keys, x, y));

        for d in &mut dist {
            *d = i16::max_value();
        }

        dist[y * width + x] = 0;
        queue.push_back((x, y, 0));
        while let Some((x, y, d)) = queue.pop_front() {
            let c = map[y * width + x];
            if ('a'..='z').contains(&c) && keys & (1 << (c as u8 - b'a' as u8)) == 0 {
                let nkeys = keys | (1 << (c as u8 - b'a' as u8));
                let p = (nkeys, x, y);
                if !visited.contains(&p) {
                    frontier.push_increase(p, Reverse(d1.0 + d));
                }
            }
            for k in 0..4 {
                let (nx, ny) = (
                    (x as isize + D[k].0) as usize,
                    (y as isize + D[k].1) as usize,
                );
                let c = map[ny * width + nx];
                if c == '#' {
                    continue;
                }
                if ('A'..='Z').contains(&c) && keys & (1 << (c as u8 - b'A' as u8)) == 0 {
                    continue;
                }
                if dist[ny * width + nx] <= d + 1 {
                    continue;
                }
                dist[ny * width + nx] = d + 1;
                queue.push_back((nx, ny, d + 1));
            }
        }
    }

    map[y_start * width + x_start] = '#';
    map[y_start * width + x_start - 1] = '#';
    map[y_start * width + x_start + 1] = '#';
    map[(y_start - 1) * width + x_start] = '#';
    map[(y_start + 1) * width + x_start] = '#';
    map[(y_start - 1) * width + x_start - 1] = '@';
    map[(y_start - 1) * width + x_start + 1] = '@';
    map[(y_start + 1) * width + x_start - 1] = '@';
    map[(y_start + 1) * width + x_start + 1] = '@';
    let mut frontier = PriorityQueue::new();
    frontier.push(
        (
            0,
            [x_start - 1, x_start - 1, x_start + 1, x_start + 1],
            [y_start - 1, y_start + 1, y_start - 1, y_start + 1],
        ),
        Reverse(0),
    );

    let mut visited = HashSet::new();
    let mut dist = vec![i16::max_value(); width * height];
    while let Some(((keys, rx, ry), d1)) = frontier.pop() {
        if keys == (1 << num_keys) - 1 {
            println!("{}", d1.0);
            break;
        }
        visited.insert((keys, rx, ry));

        for d in &mut dist {
            *d = i16::max_value();
        }

        for r in 0..4 {
            let (cx, cy) = (rx[r], ry[r]);
            dist[cy * width + cx] = 0;
            queue.push_back((cx, cy, 0));
            while let Some((x, y, d)) = queue.pop_front() {
                let c = map[y * width + x];
                if ('a'..='z').contains(&c) && keys & (1 << (c as u8 - b'a' as u8)) == 0 {
                    let nkeys = keys | (1 << (c as u8 - b'a' as u8));
                    let (mut nx, mut ny) = (rx, ry);
                    nx[r] = x;
                    ny[r] = y;
                    let p = (nkeys, nx, ny);
                    if !visited.contains(&p) {
                        frontier.push_increase(p, Reverse(d1.0 + d));
                    }
                }
                for k in 0..4 {
                    let (nx, ny) = (
                        (x as isize + D[k].0) as usize,
                        (y as isize + D[k].1) as usize,
                    );
                    let c = map[ny * width + nx];
                    if c == '#' {
                        continue;
                    }
                    if ('A'..='Z').contains(&c) && keys & (1 << (c as u8 - b'A' as u8)) == 0 {
                        continue;
                    }
                    if dist[ny * width + nx] <= d + 1 {
                        continue;
                    }
                    dist[ny * width + nx] = d + 1;
                    queue.push_back((nx, ny, d + 1));
                }
            }
        }
    }
    Ok(())
}

fn day19() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day19.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    fn run(mut vm: IntCode, x: i128, y: i128) -> bool {
        match vm.run() {
            State::Input(cookie) => cookie.set(x as i128),
            _ => unreachable!(),
        }
        match vm.run() {
            State::Input(cookie) => cookie.set(y as i128),
            _ => unreachable!(),
        }
        match vm.run() {
            State::Output(val) => {
                return val == 1;
            }
            _ => unreachable!(),
        }
    }

    let vm = IntCode::from(program);
    let mut cnt = 0;
    let (x0, y0) = (0, 0);
    let width = 50;
    let height = 50;
    for y in 0..height {
        for x in 0..width {
            if run(vm.clone(), x0 + x as i128, y0 + y as i128) {
                print!("#");
                cnt += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("{}", cnt);
    let (x0, y0) = (1509, 773);
    let mut cnt = 0;
    let width = 100;
    let height = 100;
    for y in 0..height {
        for x in 0..width {
            if run(vm.clone(), x0 + x as i128, y0 + y as i128) {
                cnt += 1;
            }
        }
    }
    println!("{} {}", cnt, x0 * 10_000 + y0);
    Ok(())
}

fn day20() -> Result<(), Box<dyn Error>> {
    let file = File::open("day20.txt")?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    let mut width = 0;
    for line in reader.lines() {
        let line = line?;
        width = line.len();
        for c in line.chars() {
            map.push(c);
        }
    }

    let height = map.len() / width;

    let mut portals = HashMap::new();
    let mut pp = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let c = map[y * width + x];
            if ('A'..='Z').contains(&c) {
                if x < width - 1 {
                    let cr = map[y * width + x + 1];
                    if ('A'..='Z').contains(&cr) {
                        if x < width - 2 && map[y * width + x + 2] == '.' {
                            if portals.contains_key(&(c, cr)) {
                                let (xx, yy) = portals[&(c, cr)];
                                pp.insert((x + 2, y), (xx, yy));
                                pp.insert((xx, yy), (x + 2, y));
                            } else {
                                portals.insert((c, cr), (x + 2, y));
                            }
                        } else if x > 1 && map[y * width + x - 1] == '.' {
                            if portals.contains_key(&(c, cr)) {
                                let (xx, yy) = portals[&(c, cr)];
                                pp.insert((x - 1, y), (xx, yy));
                                pp.insert((xx, yy), (x - 1, y));
                            } else {
                                portals.insert((c, cr), (x - 1, y));
                            }
                        }
                    }
                }
                if y < height - 1 {
                    let cb = map[(y + 1) * width + x];
                    if ('A'..='Z').contains(&cb) {
                        if y < height - 2 && map[(y + 2) * width + x] == '.' {
                            if portals.contains_key(&(c, cb)) {
                                let (xx, yy) = portals[&(c, cb)];
                                pp.insert((x, y + 2), (xx, yy));
                                pp.insert((xx, yy), (x, y + 2));
                            } else {
                                portals.insert((c, cb), (x, y + 2));
                            }
                        } else if y > 1 && map[(y - 1) * width + x] == '.' {
                            if portals.contains_key(&(c, cb)) {
                                let (xx, yy) = portals[&(c, cb)];
                                pp.insert((x, y - 1), (xx, yy));
                                pp.insert((xx, yy), (x, y - 1));
                            } else {
                                portals.insert((c, cb), (x, y - 1));
                            }
                        }
                    }
                }
            }
        }
    }
    let (x_start, y_start) = portals[&('A', 'A')];
    let (x_stop, y_stop) = portals[&('Z', 'Z')];

    let mut frontier = PriorityQueue::new();
    frontier.push((x_start, y_start), Reverse(0));
    let mut visited = HashSet::new();
    while let Some(((x, y), d1)) = frontier.pop() {
        if (x, y) == (x_stop, y_stop) {
            print!("{} ", d1.0);
            break;
        }
        visited.insert((x, y));

        for k in 0..4 {
            let (nx, ny) = (
                (x as isize + D[k].0) as usize,
                (y as isize + D[k].1) as usize,
            );
            let c = map[ny * width + nx];
            if let Some(&(nx, ny)) = pp.get(&(nx, ny)) {
                let p = (nx, ny);
                if !visited.contains(&p) {
                    frontier.push_increase(p, Reverse(d1.0 + 2));
                }
            } else if c == '.' {
                let p = (nx, ny);
                if !visited.contains(&p) {
                    frontier.push_increase(p, Reverse(d1.0 + 1));
                }
            }
        }
    }
    const D: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut frontier = PriorityQueue::new();
    frontier.push((0, x_start, y_start), Reverse(0));
    let mut visited = HashSet::new();
    while let Some(((level, x1, y1), d1)) = frontier.pop() {
        if (level, x1, y1) == (0, x_stop, y_stop) {
            println!("{}", d1.0);
            break;
        }
        visited.insert((level, x1, y1));

        if let Some(&(nx, ny)) = pp.get(&(x1, y1)) {
            let outer = x1 == 2 || x1 == width - 3 || y1 == 2 || y1 == height - 3;
            if level > 0 || !outer {
                let nlevel = if outer { level - 1 } else { level + 1 };
                let p = (nlevel, nx, ny);
                if !visited.contains(&p) {
                    frontier.push_increase(p, Reverse(d1.0 + 1));
                }
            }
        }

        for k in 0..4 {
            let (nx, ny) = (
                (x1 as isize + D[k].0) as usize,
                (y1 as isize + D[k].1) as usize,
            );
            if map[ny * width + nx] == '.' {
                let nlevel = level;
                let p = (nlevel, nx, ny);
                if !visited.contains(&p) {
                    frontier.push_increase(p, Reverse(d1.0 + 1));
                }
            }
        }
    }
    Ok(())
}

fn day21() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day21.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut vm = IntCode::from(program.clone());
    let asm = r"OR A J
AND B J
AND C J
NOT J J
AND D J
WALK
";
    let mut program_it = asm.chars();
    loop {
        match vm.run() {
            State::Output(val) => {
                if val > 0x7f {
                    println!("{}", val);
                } else {
                    print!("{}", val as u8 as char);
                }
            }
            State::Input(cookie) => match program_it.next() {
                Some(c) => {
                    cookie.set(c as i128);
                }
                None => unreachable!(),
            },
            State::Halted => break,
        }
    }

    let mut vm = IntCode::from(program);
    let asm = r"OR A J
AND B J
AND C J
NOT J J
AND D J
OR E T
OR H T
AND T J
RUN
";
    let mut program_it = asm.chars();
    loop {
        match vm.run() {
            State::Output(val) => {
                if val > 0x7f {
                    println!("{}", val);
                } else {
                    print!("{}", val as u8 as char);
                }
            }
            State::Input(cookie) => match program_it.next() {
                Some(c) => {
                    cookie.set(c as i128);
                }
                None => unreachable!(),
            },
            State::Halted => break,
        }
    }
    Ok(())
}

fn day22() -> Result<(), Box<dyn Error>> {
    let file = File::open("day22.txt")?;
    let reader = BufReader::new(file);

    let mut ops = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line == "deal into new stack" {
            ops.push((-1, -1));
        } else if line.starts_with("cut ") {
            let p = line[4..].parse::<i128>()?;
            ops.push((1, -p));
        } else {
            let i = line[line.rfind(' ').unwrap() + 1..].parse()?;
            ops.push((i, 0));
        }
    }

    pub fn modulo_mul(a: i128, b: i128, m: i128) -> i128 {
        (a * b).rem_euclid(m)
    }

    fn reduce(ops: &[(i128, i128)], n: i128) -> Shuffle {
        let mut shuffle = Shuffle::identity(n);
        for &(m, a) in ops {
            shuffle *= Shuffle { m, a, n };
        }
        shuffle
    }

    #[derive(Copy, Clone)]
    struct Shuffle {
        m: i128,
        a: i128,
        n: i128,
    }

    impl Shuffle {
        fn identity(n: i128) -> Self {
            Self { m: 1, a: 0, n }
        }

        fn apply(&self, p: i128) -> i128 {
            (p * self.m + self.a).rem_euclid(self.n)
        }
    }

    impl Mul for Shuffle {
        type Output = Self;

        fn mul(mut self, rhs: Self) -> Self::Output {
            assert_eq!(self.n, rhs.n);

            self *= rhs;
            self
        }
    }

    impl MulAssign for Shuffle {
        fn mul_assign(&mut self, rhs: Self) {
            assert_eq!(self.n, rhs.n);

            let a = (modulo_mul(self.a, rhs.m, self.n) + rhs.a).rem_euclid(self.n);
            let m = modulo_mul(self.m, rhs.m, self.n);

            self.m = m;
            self.a = a;
        }
    }

    fn pow(mut x: Shuffle, mut k: i128) -> Shuffle {
        let mut t = Shuffle::identity(x.n);
        if k == 0 {
            return t;
        }
        while k > 1 {
            if k % 2 == 1 {
                t *= x;
                x *= x;
                k /= 2;
            } else {
                x *= x;
                k /= 2;
            }
        }
        x * t
    }

    let n = 10007;
    print!("{} ", reduce(&ops, n).apply(2019));

    let n = 119315717514047;
    let shuffle = reduce(&ops, n);
    let Shuffle { m, a, .. } = pow(shuffle, 101741582076661);
    let inv = modinverse::modinverse(m, n).unwrap();
    println!("{}", modulo_mul(n + 2020 - a, inv, n));

    Ok(())
}

fn day23() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day23.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    let template = IntCode::from(program);
    let mut computers = vec![template; 50];
    for (i, vm) in computers.iter_mut().enumerate() {
        match vm.run() {
            State::Input(cookie) => cookie.set(i as i128),
            _ => unreachable!(),
        }
    }
    let mut buffers = vec![VecDeque::<(i128, i128)>::new(); computers.len()];
    let mut prev_nat_y = 0;
    let mut nat = (0, 0);
    loop {
        let mut idle = true;
        for (i, vm) in computers.iter_mut().enumerate() {
            #[derive(Copy, Clone)]
            enum ComputerState {
                Running,
                SendAddr { addr: i128 },
                SendX { addr: i128, x: i128 },
                RecvX { y: i128 },
            }

            let mut state = ComputerState::Running;
            loop {
                match (state, vm.run()) {
                    (ComputerState::Running, State::Output(addr)) => {
                        idle = false;
                        state = ComputerState::SendAddr { addr }
                    }
                    (ComputerState::SendAddr { addr }, State::Output(x)) => {
                        state = ComputerState::SendX { addr, x }
                    }
                    (ComputerState::SendX { addr, x }, State::Output(y)) => {
                        if addr == 255 {
                            if nat == (0, 0) {
                                print!("{} ", y);
                            }
                            nat = (x, y);
                        } else {
                            buffers[addr as usize].push_back((x, y));
                        }
                        break;
                    }
                    (ComputerState::Running, State::Input(cookie)) => {
                        if let Some((x, y)) = buffers[i].pop_front() {
                            idle = false;
                            cookie.set(x);
                            state = ComputerState::RecvX { y };
                        } else {
                            cookie.set(-1);
                            break;
                        }
                    }
                    (ComputerState::RecvX { y }, State::Input(cookie)) => {
                        cookie.set(y);
                        break;
                    }
                    _ => unreachable!(),
                }
            }
        }
        if idle {
            if prev_nat_y != 0 && nat.1 == prev_nat_y {
                println!("{}", nat.1);
                break;
            }
            buffers[0].push_back(nat);
            prev_nat_y = nat.1;
        }
    }
    Ok(())
}

fn day24() -> Result<(), Box<dyn Error>> {
    let file = File::open("day24.txt")?;
    let reader = BufReader::new(file);

    let mut map = [false; 25];
    let mut new = [false; 25];
    let mut levels = [[false; 25]; 401];
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            map[y * 5 + x] = c == '#';
            levels[200][y * 5 + x] = c == '#';
        }
    }
    let mut seen = HashSet::new();
    loop {
        let mut diversity = 0;
        for y in 0..5 {
            for x in 0..5 {
                let mut neigh = 0;
                if y > 0 && map[(y - 1) * 5 + x] {
                    neigh += 1;
                }
                if y < 4 && map[(y + 1) * 5 + x] {
                    neigh += 1;
                }
                if x > 0 && map[y * 5 + x - 1] {
                    neigh += 1;
                }
                if x < 4 && map[y * 5 + x + 1] {
                    neigh += 1;
                }

                new[y * 5 + x] = if map[y * 5 + x] && neigh != 1 {
                    false
                } else if !map[y * 5 + x] && (neigh == 1 || neigh == 2) {
                    true
                } else {
                    map[y * 5 + x]
                };

                if new[y * 5 + x] {
                    diversity |= 1 << (y * 5 + x);
                }
            }
        }
        map = new;
        if !seen.insert(diversity) {
            println!("{}", diversity);
            break;
        }
    }

    let mut new = [[false; 25]; 401];
    let neighbours = |x: usize, y: usize| -> &'static [(isize, usize, usize)] {
        match (x, y) {
            (0, 0) => &[(-1, 2, 1), (-1, 1, 2), (0, 1, 0), (0, 0, 1)],
            (1, 0) => &[(-1, 2, 1), (0, 0, 0), (0, 2, 0), (0, 1, 1)],
            (2, 0) => &[(-1, 2, 1), (0, 1, 0), (0, 3, 0), (0, 2, 1)],
            (3, 0) => &[(-1, 2, 1), (0, 2, 0), (0, 4, 0), (0, 3, 1)],
            (4, 0) => &[(-1, 2, 1), (-1, 3, 2), (0, 3, 0), (0, 4, 1)],
            (0, 4) => &[(-1, 2, 3), (-1, 1, 2), (0, 0, 3), (0, 1, 4)],
            (1, 4) => &[(-1, 2, 3), (0, 1, 3), (0, 0, 4), (0, 2, 4)],
            (2, 4) => &[(-1, 2, 3), (0, 2, 3), (0, 1, 4), (0, 3, 4)],
            (3, 4) => &[(-1, 2, 3), (0, 3, 3), (0, 2, 4), (0, 4, 4)],
            (4, 4) => &[(-1, 2, 3), (-1, 3, 2), (0, 4, 3), (0, 3, 4)],
            (0, 1) => &[(-1, 1, 2), (0, 0, 0), (0, 0, 2), (0, 1, 1)],
            (0, 2) => &[(-1, 1, 2), (0, 0, 1), (0, 0, 3), (0, 1, 2)],
            (0, 3) => &[(-1, 1, 2), (0, 0, 2), (0, 0, 4), (0, 1, 3)],
            (4, 1) => &[(-1, 3, 2), (0, 4, 0), (0, 4, 2), (0, 3, 1)],
            (4, 2) => &[(-1, 3, 2), (0, 4, 1), (0, 4, 3), (0, 3, 2)],
            (4, 3) => &[(-1, 3, 2), (0, 4, 2), (0, 4, 4), (0, 3, 3)],
            (1, 1) => &[(0, 1, 0), (0, 1, 2), (0, 0, 1), (0, 2, 1)],
            (3, 1) => &[(0, 3, 0), (0, 3, 2), (0, 2, 1), (0, 4, 1)],
            (1, 3) => &[(0, 1, 2), (0, 1, 4), (0, 0, 3), (0, 2, 3)],
            (3, 3) => &[(0, 3, 2), (0, 3, 4), (0, 2, 3), (0, 4, 3)],
            (2, 1) => &[
                (1, 0, 0),
                (1, 1, 0),
                (1, 2, 0),
                (1, 3, 0),
                (1, 4, 0),
                (0, 1, 1),
                (0, 3, 1),
                (0, 2, 0),
            ],
            (2, 3) => &[
                (1, 0, 4),
                (1, 1, 4),
                (1, 2, 4),
                (1, 3, 4),
                (1, 4, 4),
                (0, 1, 3),
                (0, 3, 3),
                (0, 2, 4),
            ],
            (1, 2) => &[
                (1, 0, 0),
                (1, 0, 1),
                (1, 0, 2),
                (1, 0, 3),
                (1, 0, 4),
                (0, 1, 1),
                (0, 1, 3),
                (0, 0, 2),
            ],
            (3, 2) => &[
                (1, 4, 0),
                (1, 4, 1),
                (1, 4, 2),
                (1, 4, 3),
                (1, 4, 4),
                (0, 3, 1),
                (0, 3, 3),
                (0, 4, 2),
            ],
            _ => unreachable!(),
        }
    };
    for gen in 0..200 {
        for level in 200 - gen - 1..=200 + gen + 1 {
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let mut neigh = 0;
                    let curr = levels[level][y * 5 + x];
                    for &(dl, nx, ny) in neighbours(x, y) {
                        if dl == -1 && level == 0 || dl == 1 && level == 400 {
                            continue;
                        }
                        let nl = (level as isize + dl) as usize;
                        neigh += levels[nl][ny * 5 + nx] as i32;
                    }

                    new[level][y * 5 + x] = if curr && neigh != 1 {
                        false
                    } else if !curr && (neigh == 1 || neigh == 2) {
                        true
                    } else {
                        curr
                    };
                }
            }
        }
        levels = new;
    }
    let mut bugs = 0;
    for level in 0..=400 {
        for i in 0..25 {
            if i != 12 && levels[level][i] {
                bugs += 1;
            }
        }
    }
    println!("{}", bugs);
    Ok(())
}

fn day25() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("day25.txt")?
        .trim_end()
        .split(',')
        .map(|v| v.parse::<i128>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut vm = IntCode::from(program);
    let playthrough = r"
east
take sand
west
west
north
take wreath
east
take fixed point
west
south
south
east
east
east
take space law space brochure
south
south
west
";
    let mut it = playthrough.chars();
    loop {
        match vm.run() {
            State::Output(val) => {
                let c = val as u8 as char;
                print!("{}", c);
            }
            State::Input(cookie) => {
                if let Some(c) = it.next() {
                    cookie.set(c as i128);
                } else {
                    break;
                }
            }
            State::Halted => break,
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    day20()
}
