use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::io::{Write, stdin, stdout};
use std::cmp::Reverse;

fn quotients_dhondts(n: u64) -> impl Iterator<Item = u64> {
    (1..).map(move |d| n / d)
}

fn seats(total_seats: u64, votes: &[u64]) -> Vec<u64> {
    if votes.len() == 1 {
        // If there's only one party, they get all the seats
        return vec![total_seats];
    }

    let mut quotient_makers: Vec<_> = votes.into_iter().map(|&n| quotients_dhondts(n)).collect();
    let mut quotients = vec![0; votes.len()];
    let mut seats = vec![0; votes.len()];

    for (quotient_maker, quotient) in quotient_makers.iter_mut().zip(quotients.iter_mut()) {
        *quotient = quotient_maker.next().unwrap();
    }

    for _ in 0..total_seats {
        let (quotient, (seats,  quotient_maker)) = quotients
            .iter_mut()
            .zip(seats.iter_mut().zip(quotient_makers.iter_mut()))
            .max_by_key(|(q, _)| **q)
            .unwrap();

        *seats += 1;
        *quotient = quotient_maker.next().unwrap();
    }

    seats
}
fn prompt(prompt: impl Display) -> String {
    let mut s = String::new();
    let prompt = format!("{}: ", prompt);
    print!("{}", prompt);
    stdout().flush().unwrap();

    stdin().read_line(&mut s).unwrap();
    if s.starts_with(&prompt) {
        s = s[prompt.len()..].to_owned();
    }
    s
}

fn main() {
    let coalitions = parse_coalitions(prompt("Coalitions").trim());
    let total_seats = prompt("Total seats").trim().parse().unwrap();
    let mut votes = HashMap::new();

    for party in coalitions.iter().flat_map(|v| v.iter().flat_map(|s| s.chars())) {
        votes.insert(party, prompt(party).trim().parse().unwrap());
    }

    let mut seats = calculate_seats(total_seats, &votes, &coalitions);

    println!();
    println!("Coalitions:");
    for mega_coalition in coalitions {
        let display_name = mega_coalition.join("+");
        if display_name.chars().count() == 1 {
            continue;
        }
        let get_name: String = mega_coalition.iter().flat_map(|s| s.chars()).collect();
        println!(" {}: {}", display_name, seats.remove(&get_name).unwrap());
        if mega_coalition.len() > 1 {
            for coalition in mega_coalition {
                if coalition.chars().count() > 1 {
                    println!("  {}: {}", coalition, seats.remove(&coalition).unwrap());
                }
            }
        }
    }
    println!("Parties:");
    let mut party_seats: Vec<_> = seats.into_iter().collect();
    party_seats.sort_by_key(|&(_, seats)| Reverse(seats));

    for (party, seats) in party_seats {
        println!(" {}: {}", party, seats);
    }
}

fn parse_coalitions(s: &str) -> Vec<Vec<String>> {
    let mut coalitions = vec![vec![String::new()]];

    for c in s.chars() {
        match c {
            ' ' | ',' => {
                if !coalitions.last().unwrap().last().unwrap().is_empty() {
                    coalitions.push(vec![String::new()]);
                }
            }
            '+' => {
                let last = coalitions.last_mut().unwrap();
                assert!(!last.last().unwrap().is_empty());
                last.push(String::new());
            }
            c => {
                coalitions.last_mut().unwrap().last_mut().unwrap().push(c);
            }
        }
    }

    coalitions
}

fn calculate_seats(total_seats: u64, votes: &HashMap<char, u64>, coalitions: &Vec<Vec<String>>) -> BTreeMap<String, u64> {
    let mut seat_distribution = BTreeMap::new();

    let mut mega_coalition_votes = Vec::with_capacity(coalitions.len());
    let mut mega_coalition_names = Vec::with_capacity(coalitions.len());

    for mega_coalition in coalitions {
        let s: String = mega_coalition.iter().flat_map(|v| v.chars()).collect();
        seat_distribution.insert(s.clone(), 0);
        mega_coalition_votes.push(s.chars().map(|c| votes[&c]).sum());
        mega_coalition_names.push(s);
    }

    let mega_coalition_seats = seats(total_seats, &mega_coalition_votes);
    drop(mega_coalition_votes);
    for (name, seats) in mega_coalition_names.into_iter().zip(mega_coalition_seats) {
        seat_distribution.insert(name, seats);
    }

    let mut final_coalitions = Vec::new();

    for mega_coalition in coalitions {
        let mega_name: String = mega_coalition.iter().flat_map(|v| v.chars()).collect();
        let total_seats = seat_distribution[&mega_name];

        let votes: Vec<_> = mega_coalition.iter().map(|coalition| coalition.chars().map(|c| votes[&c]).sum()).collect();

        let seats = seats(total_seats, &votes);

        for (coalition, seats) in mega_coalition.into_iter().zip(seats) {
            final_coalitions.push(coalition.clone());
            if let Some(seats_before) = seat_distribution.insert(coalition.clone(), seats) {
                debug_assert_eq!(seats_before, seats)
            }
        }
    }

    for final_coalition in final_coalitions {
        let votes: Vec<_> = final_coalition.chars().map(|c| votes[&c]).collect();
        let seats = seats(seat_distribution[&final_coalition], &votes);

        for (party, seats) in final_coalition.chars().zip(seats) {
            if let Some(seats_before) = seat_distribution.insert(party.to_string(), seats) {
                debug_assert_eq!(seats, seats_before);
            }
        }
    }

    seat_distribution
}

/*
A CV+DO BFØ
21
21343
3163
1902
901
217
2163
3502
1502
*/