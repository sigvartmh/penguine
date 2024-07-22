#[derive(Debug)]
struct Player<'a> {
  name: &'a str,
  score: usize
}

struct Team<'a> {
  players: [Player<'a>; 2]
}
struct Courts<'a> {
  name: &'a str,
}
struct CourtGame<'a> {
  teams: [Team<'a>; 2]
}

struct Round<'a> {
  court: Vec<CourtGame<'a>>
}

struct Game <'a>{
  players: Vec<Player<'a>>,
  rounds: Vec<Round<'a>>
}

impl Player<'_> {
  fn add_score(&mut self, score: usize) {
    self.score = self.score + score;
  }
}

impl Game<'_> {
  fn new<'a>(players: Vec<Player<'a>>) -> Game<'a> {
    Game{players: players,
    rounds: Vec::<Round>::new()}
  }
}

fn main() {
  let mut players = Vec::<Player>::new();
  players.push(Player{ name: "Sigvart", score: 0});
  players.push(Player{ name: "Emanuel", score: 0});
  players.push(Player{ name: "Jens", score: 0});
  players.push(Player{ name: "Martin", score: 0});

  println!("{:?}", players);
  for player in &mut players {
    player.add_score(8);
  }
  println!("{:?}", players);
  let mut game = Game::new(players);
  for player in &mut game.players {
    player.add_score(10)
  }
  println!("{:?}", game.players);


  println!("Hello, world!");
}
