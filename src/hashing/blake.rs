use self::super::hash_string;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use blake::Blake;


hash_func!(Blake::new(512).unwrap(),
           |blake: &mut Blake, buffer: &[u8], read: usize| blake.update(&buffer[0..read]),
           |blake: Blake| {
               let mut result = [0; 64];
               blake.finalise(&mut result);
               hash_string(&result)
           });
