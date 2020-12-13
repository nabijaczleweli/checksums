use self::super::hash_string;
use blake::Blake;


hash_func_write!(Blake::new(512).unwrap(),
                 |mut blake: Blake| {
                     let mut result = [0; 64];
                     blake.finalise(&mut result);
                     hash_string(&result)
                 });
