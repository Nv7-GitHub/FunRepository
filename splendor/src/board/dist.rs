use super::*;

fn recalc_reqs(reqs: &mut Vec<Requirement>) {
  loop {
    let mut done = true;
    for i in 0..reqs.len() {
      if reqs[i].count == 0 {
        reqs.remove(i);
        done = false;
        break;
      }
    }
    if done {
      return;
    }
  }
}

impl Board {
  pub fn distance(&self, card: &Card, cards: &Vec<Card>) -> Option<usize> {
    let mut reqs = card.requirements.clone();
    for c in cards.iter() {
      if let Some(r) = reqs.iter_mut().find(|r| r.gem == c.gem) {
        if r.count > 0 {
          r.count -= 1;
        }
      }
    }
    recalc_reqs(&mut reqs);

    // Check if possible
    let total: usize = reqs.iter().map(|v| v.count).sum();
    if total > MAX_GEMS {
      return None;
    }

    let mut cnt = 0;

    // Get 3 at a time until you can't
    while reqs.len() > 0 {
      if reqs.len() >= 3 {
        // Remove 3 at a time
        for (i, req) in reqs.iter_mut().enumerate() {
          if i >= 3 {
            break;
          }
          req.count -= 1;
        }
      } else {
        // Remove 2 at a time
        if reqs[0].count >= 4 {
          reqs[0].count -= 2;
        } else {
          reqs[0].count -= 1;
        }
      }
      recalc_reqs(&mut reqs);

      cnt += 1;
    }

    // Return
    Some(cnt)
  }
}