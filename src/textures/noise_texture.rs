use crate::{
    textures::{
        Texture,
        TextureCoordinates,
    },
    math::{
        self,
        Point3,
        Vec3,
    },
    random,
};

#[derive(Copy, Clone)]
pub struct NoiseTexture {
    data: &'static PerlinData,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        lazy_static! {
            static ref PERLIN_DATA: PerlinData = PerlinData::new();
        }
        Self {
            data: &PERLIN_DATA,
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: &TextureCoordinates, p: &Point3) -> Vec3 {
        let depth = 7;
        let result = self.scale*p.z + 10.0*self.data.turb(p, depth);
        let result = result.sin() + 1.0;
        result * math::vec3(0.5, 0.5, 0.5)
    } 
}

struct PerlinData {
    random_unit_vectors: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl PerlinData {
    pub fn new() -> Self {
        Self {
            random_unit_vectors: random_255_unit_vectors(),
            perm_x: random_permutation_0_to_255(),
            perm_y: random_permutation_0_to_255(),
            perm_z: random_permutation_0_to_255(),
        }
    }

    pub fn turb(&self, p: &Point3, depth: u32) -> f32 {
        let mut acc = 0.0;
        let mut point = *p;
        let mut weight = 1.0;
        
        for _ in 0..depth {
            acc += weight*self.noise(&point);
            weight *= 0.5;
            point *= 2.0;
        }
    
        acc.abs()
    }

    fn noise(&self, p: &Point3) -> f32 {
        let q = p.map(|c| c - c.floor());
        let (u, v, w) = (q.x, q.y, q.z);

        let r = p.map(|c| c.floor())
                 .map(|c| if c > 0.0 { c } else { 0.0 });
        let i = r.x as usize;
        let j = r.y as usize;
        let k = r.z as usize;
        let cube = self.random_unit_vecs_cube(i, j, k);

        perlin_interpolation(&cube, u, v, w)
    }

    fn random_unit_vecs_cube(&self, i: usize, j: usize, k: usize) -> [[[Vec3; 2]; 2]; 2] {
        let mut result = [[[math::vec3(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    result[di][dj][dk] = self.random_unit_vectors[
                        self.perm_x[(i+di) & 255] ^
                        self.perm_y[(j+dj) & 255] ^
                        self.perm_z[(k+dk) & 255]
                    ];
                }
            }
        }
        result
    }
}

fn random_255_unit_vectors() -> Vec<Vec3> {
    use math::InnerSpace;
    let rand = || 2.0*random::random_float_from_0_to_1() - 1.0;
    (0..256)
    .map(|_| {
        math::vec3(rand(), rand(), rand()).normalize()
    })
    .collect()
}

fn random_permutation_0_to_255() -> Vec<usize> {
    let mut result = (0..256).collect::<Vec<usize>>();
    permute(&mut result);
    result
}

fn permute(vec: &mut Vec<usize>) {
    use random::random_float_from_0_to_1;
    
    for i in (1..vec.len()).rev() {
        let target = random_float_from_0_to_1()*(i as f32 + 1.0);
        let target = target.floor() as usize;
        vec.as_mut_slice().swap(i, target);
    }
}

fn perlin_interpolation(coefficients: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let p = Point3::new(u, v, w).map(|c| c*c*(3.0 - 2.0*c));
    let (uu, vv, ww) = (p.x, p.y, p.z);
    let mut acc = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let (ii, jj, kk) = (i as f32, j as f32, k as f32);
                let weight_v = math::vec3(u - ii, v - jj, w - kk);
                acc += (ii*uu + (1.0 - ii)*(1.0 - uu))*
                       (jj*vv + (1.0 - jj)*(1.0 - vv))*
                       (kk*ww + (1.0 - kk)*(1.0 - ww))*
                       math::dot(coefficients[i][j][k], weight_v);
            }
        }
    }

    acc
}