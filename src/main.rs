use std::collections::HashMap;
use std::collections::BTreeSet;
use std::cmp;
use std::cmp::Ordering;

type RowID = u64;
type UserName = String;
type Score = u64;

#[derive(Debug, Clone)]
struct DataRow {
    username: UserName,
    score: Score
}

#[derive(Eq, PartialOrd, PartialEq)]
struct ScoreIdxRow {
    score: Score,
    row_id: RowID,
}

impl Ord for ScoreIdxRow {
  //I'm willing to bet there's something in std that does this for me
  fn cmp(&self, other: &Self) -> Ordering {
    match self.score.cmp(&other.score) {
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => self.row_id.cmp(&other.row_id),
        Ordering::Greater => Ordering::Less
    }
  }
}


struct ScoreBoard {
    //owner of data
    rows: HashMap<RowID, DataRow>,
    
    //doesn't own
    user_idx: HashMap<UserName, RowID>,
    score_idx: BTreeSet<ScoreIdxRow>,
    next_row_id: RowID,
}

impl ScoreBoard {
    fn new() -> ScoreBoard {
        ScoreBoard {
            user_idx: HashMap::new(),
            score_idx: BTreeSet::new(),
            rows: HashMap::new(),
            next_row_id: 0,
        }

    }

    fn insert_user(& mut self, user: &str, score: Score) -> RowID {
        if let Some(row_id) = self.user_idx.get(user) {
          if let Some(data_row) = self.rows.get(row_id) {
            self.score_idx.remove( &ScoreIdxRow {
                score: data_row.score,
                row_id: *row_id
              });  
          }
          self.rows.remove(row_id);
        }
        self.next_row_id += 1;
        self.rows.insert(self.next_row_id, DataRow { username: String::from(user), score}); //copy user string makes sense
        self.user_idx.insert(String::from(user), self.next_row_id); // another user copy
        self.score_idx.insert(ScoreIdxRow {score, row_id: self.next_row_id});
        return self.next_row_id;
    }

    fn delete_user(& mut self, user: &str) -> Option<RowID>{
        let mut ret = None;
        if let Some(row_id) =  self.user_idx.get(user) {
            ret = Some(*row_id);
            if let Some(row) =  self.rows.get(row_id) {
              self.score_idx.remove(&ScoreIdxRow { score: row.score, row_id: *row_id});
            }
            self.rows.remove(&row_id);
            self.user_idx.remove(user);
        }
        ret
    }

    fn top_n(&self, n: usize) -> Vec<DataRow> {
        let n = cmp::min(n, self.rows.len());
        let mut vec = Vec::new();
        let mut iter = self.score_idx.iter();
        for _ in 0..n {
            let score_idx_row = iter.next().unwrap();
            if let Some(row) = self.rows.get(&score_idx_row.row_id) {
                vec.push(row.clone())
            }
        }
        vec
    }
}

fn main() {
    println!("Hello, world!");
    let mut score_board = ScoreBoard::new();
    println!("matt id {:?}",score_board.insert_user("matt", 1200));
    println!("alex id {:?}",score_board.insert_user("alex", 1500));
    println!("charlie id {:?}",score_board.insert_user("charlie", 1300));

    println!("top 2:");
    for elem in score_board.top_n(2).iter() {
        println!("{:?}", elem);
    }

    println!("deleted alex id {:?}",score_board.delete_user("alex"));
    println!("top 2:");
    for elem in score_board.top_n(2).iter() {
        println!("{:?}", elem);
    }
}
