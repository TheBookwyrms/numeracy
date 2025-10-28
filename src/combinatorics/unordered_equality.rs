pub fn unordered_equality<T:Clone+PartialEq>(v1:Vec<T>, v2:Vec<T>) -> bool {
    match v1.len() == v2.len() {
        false => false,
        true => {
            let iter_vec = v1;
            let mut remove_from_vec = v2;

            for val in iter_vec {
                let mut counted = false;
                for (i, val2) in remove_from_vec.clone().iter().enumerate() {
                    if (&val==val2) && !counted {
                        remove_from_vec.remove(i);
                        counted = true;
                    }
                }
            }

            match remove_from_vec.len() {
                0 => true,
                _ => false,
            }
        },
    }
}