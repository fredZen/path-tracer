use super::prelude::*;
use std::{collections::HashMap, default::Default, fmt::Debug, hash::Hash, ops::AddAssign};

#[derive(Debug, Default)]
pub struct Record {
    hits: usize,
    misses: usize,
}

#[derive(Debug)]
pub struct Stats<C: Debug + Eq + Hash> {
    by_category: HashMap<C, Record>,
}

impl<C: Debug + Eq + Hash> Stats<C> {
    pub fn new() -> Self {
        Self {
            by_category: HashMap::new(),
        }
    }

    fn entry(&mut self, cat: C) -> &mut Record {
        self.by_category.entry(cat).or_insert_with(Default::default)
    }

    fn hit(&mut self, cat: C) {
        self.entry(cat).hits += 1;
    }

    fn miss(&mut self, cat: C) {
        self.entry(cat).misses += 1;
    }
}

impl<C: Debug + Eq + Hash> AddAssign for Stats<C> {
    fn add_assign(&mut self, rhs: Self) {
        for (c, r) in rhs.by_category {
            let e = self.entry(c);
            e.hits += r.hits;
            e.misses += r.misses;
        }
    }
}

pub struct StatsRecorder<C: Copy, H> {
    hitable: H,
    category: C,
}

impl<C: Copy + Eq + Hash, H> StatsRecorder<C, H> {
    pub fn new(category: C, hitable: H) -> Self {
        Self { hitable, category }
    }
}

impl<C: Copy + Debug + Eq + Hash, H: Hitable<Stats<C>>> Hitable<Stats<C>> for StatsRecorder<C, H> {
    fn hit(&self, c: &mut Stats<C>, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let res = self.hitable.hit(c, r, t_min, t_max);
        if res.is_some() {
            c.hit(self.category)
        } else {
            c.miss(self.category)
        }
        res
    }

    fn bounding_box(&self, t0: Float, t1: Float) -> Option<Cow<BoundingBox>> {
        self.hitable.bounding_box(t0, t1)
    }
}
