mod rock_paper_scissors;

fn main() {
    println!(
        "{:?}",
        rock_paper_scissors::rock_paper_scissors([['r', 'r'].to_vec()].to_vec())
    );
    println!(
        "{:?}",
        rock_paper_scissors::rock_paper_scissors([['r', 's'].to_vec()].to_vec())
    );
    println!(
        "{:?}",
        rock_paper_scissors::rock_paper_scissors([['s', 'r'].to_vec()].to_vec())
    );
    println!(
        "{:?}",
        rock_paper_scissors::rock_paper_scissors(
            [
                ['r', 'r'].to_vec(),
                ['l', 's'].to_vec(),
                ['s', 'r'].to_vec(),
                ['p', 'o'].to_vec(),
                ['o', 'l'].to_vec(),
            ]
            .to_vec()
        )
    );
}
