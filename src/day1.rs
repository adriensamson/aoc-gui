use crate::AocSender;

pub(super) fn day1(sender: AocSender, input: String) {
    let instructions : Vec<i16> = input.lines().filter_map(|l| {
        if let Some(n) = l.strip_prefix('R') {
            n.parse().ok()
        } else if let Some(n) = l.strip_prefix('L') {
            n.parse().ok().map(|n: i16| -n)
        } else {
            None
        }
    }).collect();

    let mut dial = 50;
    let mut count0 = 0usize;
    for &n in &instructions {
        dial += n + 100;
        dial %= 100;
        if dial == 0 {
            count0 += 1;
        }
    }
    sender.result_part1(count0);

    let mut dial = 50;
    let mut count0 = 0usize;
    for &n in &instructions {
        let from = dial;
        if n > 0 {
            dial += n;
            while dial >= 100 {
                dial -= 100;
                count0 += 1;
            }
        } else if n < 0 {
            if from == 0 {
                count0 -= 1;
            }
            dial += n;
            while dial < 0 {
                dial += 100;
                count0 += 1;
            }
            if dial == 0 {
                count0 += 1;
            }
        }
    }
    sender.result_part2(count0);
}