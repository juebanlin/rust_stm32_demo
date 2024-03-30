#[cfg(test)]
mod tests{
    use crate::simpleAoi::{Aoi, Entity, run};
    use rand::Rng;
    use std::time::Instant;

    #[test]
    fn test() {
        let r = Aoi::forInGrid2(25f32, 20f32, 6f32, 6f32, 25f32, 25f32, 4, 4);
        println!("{:?}", r);
    }

    #[test]
    fn testaoi() {
        fn buildEntitys(
            num: i32,
            worldX: f32,
            worldY: f32,
            rangeMin: f32,
            rangeMax: f32,
        ) -> Vec<Entity> {
            let mut rng = rand::thread_rng();
            let mut vec = Vec::<Entity>::with_capacity(num as usize);
            for i in 0..num {
                let r = rng.gen::<f32>() * (rangeMax - rangeMin) + rangeMin;
                let e = Entity::new(rng.gen::<f32>() * worldX, rng.gen::<f32>() * worldY, r);
                vec.push(e);
            }
            vec
        }
        let input = buildEntitys(40000, 5120f32, 5120f32, 3f32, 15f32);
        println!("start");
        let time = Instant::now();
        let r = run(input);
        let useTime = time.elapsed().as_millis();
        println!("{},{}", r.len(), useTime);
    }
}