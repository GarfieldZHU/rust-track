pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() <= 0 {
        return minefield.iter().map(|r| r.to_string()).collect::<Vec<_>>();
    }

    let mut extended = minefield.iter().map(|r| format!(" {} ", r)).collect::<Vec<_>>();
    let n = extended[0].len();
    let padding_top = " ".repeat(n);
    let padding_bottom = " ".repeat(n);
    extended.insert(0, padding_top);
    extended.push(padding_bottom);

    let mut result = vec![];
    for i in 1..extended.len()-1 {
        let mut line = String::from("");
        for j in 1..extended[i].len()-1 {
            let ch = extended[i].chars().collect::<Vec<_>>()[j];
            if ch == ' ' {
                let mut count = 0;
                for x in 0..3 {
                    for y in 0..3 {
                        let to_check = extended[i+x-1].chars().collect::<Vec<_>>()[j+y-1];
                        if to_check == '*' {
                            count += 1;
                        }
                    }
                }
                if count == 0 { line.push(' '); } else { line.push_str(&count.to_string()) };
            } else {
                line.push(ch);
            }
        }
        result.push(line);
    }

    result
}
