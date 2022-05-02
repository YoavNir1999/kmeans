//use rand::Rng;
//use rayon::iter::*;

#[derive(Copy,Clone,Debug)]
pub struct Point {
    pub r:f64,
    pub g:f64,
    pub b:f64
}

impl Point {
    fn distance(&self,other:&Point) -> f64 {
        return (
            (self.r-other.r).powf(2.0)+
            (self.g-other.b).powf(2.0)+
            (self.b-other.b).powf(2.0)
        ).sqrt()
    }

    pub fn point_to_pixel(&self) -> [u8;3] {
        return [self.r as u8, self.g as u8, self.b as u8]
    }
}

#[derive(Debug)]
pub struct Kmeans {
    k:usize,
    data:Vec<Point>,
    pub means:Vec<Point>
}

impl PartialEq for Point {
    fn eq(&self,other:&Point) -> bool {
        return self.r==other.r &&
        self.g==other.g &&
        self.b==other.b
    }
}

impl Kmeans {
    pub fn new(data:Vec<Point>,k:usize) -> Kmeans {
        //let mut rng = rand::thread_rng();
        let mut means = Vec::new();
        //let mut previus_rands = Vec::new();
        for i in 0..k {
            /*
            loop {
                let rand_point : usize = rng.gen_range(0..data.len());
                if !previus_rands.contains(&rand_point) {
                    previus_rands.push(rand_point);
                    break
                } 
            }*/
            
            means.push(Point {
                r : i as f64 *(255.0/k as f64),
                g : i as f64 *(255.0/k as f64),
                b : i as f64 *(255.0/k as f64)
            });
                    
        }
        return Kmeans {
            k:k,
            data:data,
            means:means
        }
    }

    

    pub fn new_means(&mut self) -> f64 {
        let mut diff = 0.0;
        let mut partitions = vec!([0.0,0.0,0.0,0.0];self.k);
        for p_idx in 0..self.data.len() {
            let mut closest_idx = 0;
            let mut closest_distance = self.data[p_idx].distance(&self.means[0]);
            for m_idx in 1..self.means.len() {
                let new_dist = self.data[p_idx].distance(&self.means[m_idx]);
                if new_dist<closest_distance {
                    closest_distance=new_dist;
                    closest_idx=m_idx;
                };
            }
            partitions[closest_idx][0]+=1.0;
            partitions[closest_idx][1]+=self.data[p_idx].r;
            partitions[closest_idx][2]+=self.data[p_idx].g;
            partitions[closest_idx][3]+=self.data[p_idx].b;
        }
        for idx in 0..self.k {
            let new_mean = Point {
                r:partitions[idx][1]/partitions[idx][0],
                g:partitions[idx][2]/partitions[idx][0],
                b:partitions[idx][3]/partitions[idx][0]
            };
            diff += new_mean.distance(&self.means[idx]);
            self.means[idx] = new_mean;
        }

        return diff
    }


}
/*
trait CalcMean {
    fn calc_mean(&self) -> Point;
}

impl CalcMean for Vec<Point> {
    fn calc_mean(&self) -> Point {
        let mean_len = self.len() as f64;
        let mut total_r = 0.0;
        let mut total_g = 0.0;
        let mut total_b = 0.0;
        for point in self {
            total_r+=point.r/mean_len;
            total_g+=point.g/mean_len;
            total_b+=point.b/mean_len;
        }
        return Point {
            r:total_r,
            g:total_g,
            b:total_b
        }
    }
}
*/
