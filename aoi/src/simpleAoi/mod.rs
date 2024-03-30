#![feature(slice_ptr_len)]
#![feature(slice_ptr_get)]

pub use crate::simpleAoi::aoi::{Entity, Aoi};//向上导出

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod ffi;
mod aoi;

pub fn run(input: Vec<Entity>) -> Vec<i32> {
    let num = input.len();
    let mut aoi = Aoi::new(num as i32, 5120f32, 5120f32, 32f32, 32f32);
    let mut id = 0;
    for x in input {
        aoi.enter(x, id);
        id += 1;
    }
    let result = aoi.buildResult();
    println!("rust group size：{}", result.len());
    let mut r = Vec::<i32>::new(); //组名,id,组名,id
    let mut gnum = 0;
    for (i, group) in result.iter().enumerate() {
        let gid = i as i32;
        gnum += 1;
        for x in group {
            r.push(gid);
            r.push(*x);
        }
    }
    r
}


