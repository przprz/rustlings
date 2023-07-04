// iterators5.rs
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a hint.


use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn imperative_count(project_progress: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in project_progress.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn functional_count(project_progress: &HashMap<String, Progress>, value: Progress) -> usize {
    // map = { "variables1": Complete, "from_str": None, ... }
   project_progress
       .values()
       .filter(|&p| p == &value)
       .count()
}

fn imperative_count_collection(group_project_progress: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for project_progress in group_project_progress {
        for val in project_progress.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn functional_count_collection(group_project_progress: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
group_project_progress
    .iter()
    .map(|project_progress| functional_count(&project_progress, value))
    .sum()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, functional_count(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, functional_count(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, functional_count(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                imperative_count(&map, progress_state),
                functional_count(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            functional_count_collection(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, functional_count_collection(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, functional_count_collection(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                imperative_count_collection(&collection, progress_state),
                functional_count_collection(&collection, progress_state)
            );
        }
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
