// hashmaps3.rs

// 给定一个足球赛的得分列表（每个一行）。
// 每行的格式如下：
// <队伍1名字>,<队伍2名字>,<队伍1得分>,<队伍2得分>
// 比如：英国,法国,4,2（英国得了4分，法国得了2分）。

// 你需要构建一个得分表，包含队伍的名字、队伍的得分，以及队伍的丢分。
// 一个构建得分表的方法是使用哈希表。
// 使用哈希表的解法有部分已经写好了，完成它以通过测试。

// 让我通过测试！

// 执行 `rustlings hint hashmaps3` 或在观察模式下使用 `hint` 子命令来获取提示。

// // I AM NOT DONE

use std::collections::HashMap;

// 一个存储队伍名字和它的得分详情的结构。
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // 队伍的名字是键，它关联的结构体是值。
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: 使用从当前行提取的详细信息填充得分表。
        // 请记住，队伍1的得分将会是队伍2的丢分，
        // 同样，队伍2的得分也将会是队伍1的丢分。
        // let insert_scores = |name, scores_1, scores_2| scores

        let new_team = |name, goals_scored, goals_conceded| Team {
            name,
            goals_scored,
            goals_conceded,
        };

        let updated = |scores: &mut HashMap<String, Team>, name: String, goals_scored, goals_conceded| {
            let team_info =
                scores
                    .entry(name.clone())
                    .or_insert(new_team(name.clone(), 0, 0));
            *team_info = new_team(
                name,
                team_info.goals_scored + goals_scored,
                team_info.goals_conceded + goals_conceded,
            );
        };

        updated(&mut scores, team_1_name, team_1_score, team_2_score);
        updated(&mut scores, team_2_name, team_2_score, team_1_score);
        // let team_1_info =
        //     scores
        //         .entry(team_1_name.clone())
        //         .or_insert(new_team(team_1_name.clone(), 0, 0));
        // *team_1_info = new_team(
        //     team_1_name,
        //     team_1_info.goals_scored + team_1_score,
        //     team_1_info.goals_conceded + team_2_score,
        // );
        // let team_2_info =
        //     scores
        //         .entry(team_2_name.clone())
        //         .or_insert(new_team(team_2_name.clone(), 0, 0));
        // *team_2_info = new_team(
        //     team_2_name,
        //     team_2_info.goals_scored + team_2_score,
        //     team_2_info.goals_conceded + team_1_score,
        // );
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
