use self::super::hash_string;
use blake::Blake;


hash_func!(Blake::new(512).unwrap(),
           |blake: &mut Blake, buffer: &[u8]| blake.update(buffer),
           |blake: Blake| {
               let mut result = [0; 64];
               blake.finalise(&mut result);
               hash_string(&result)
           });
