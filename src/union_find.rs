use std::marker::PhantomData;

macro_rules! generate_qf_uf_impl {
    ($($num_type:ident), *) => {
        $(
        impl UnionFind<QuickFind, $num_type> {
            pub fn find(&self, data: $num_type) -> $num_type {
                self.id[data as usize]
            }

            pub fn union(&mut self, p: $num_type, q: $num_type) {
                // decrement the counter if successful
                // get representative node for p
                let representative_p = self.id[p as usize];
                let representative_q = self.id[q as usize];
                // change all elements in buffer which is represented by p to be represented by q
                for item in &mut self.id {
                    if *item == representative_p {
                        *item = representative_q;
                    }
                }
                self.count -= 1;
            }
            
            pub fn connected(&self, p: $num_type, q: $num_type) -> bool {
                self.find(q) == self.find(p)
            }
        }
        )*
    };
}


macro_rules! generate_wqu_uf_impl {
    ($($num_type:ident), *) => {
        $(
        impl UnionFind<WeightedQuickUnion, $num_type, Vec<$num_type>> {
            
            pub fn new(n: $num_type) -> Self {
                Self {
                    id: (0..n).collect(),
                    phantom: PhantomData::default(),
                    count: n as usize,
                    rank: vec![1; n as usize]
                }
            }

            pub fn find(&self, mut i: $num_type) -> $num_type {
                while i != self.id[i as usize] {
                    i = self.id[i as usize]
                }
                i
            }

            pub fn union(&mut self, p: $num_type, q: $num_type) {
                // find root of p and root of q and connect p to q
                // ie. making the root q the new root of p
                let root_p = self.find(p);
                let root_q = self.find(q);
                
                if root_p == root_q {
                    return;
                }

                if self.rank[root_p as usize] < self.rank[root_q as usize] {
                    self.id[root_p as usize] = root_q;
                    self.rank[root_q as usize] += self.rank[root_p as usize];
                } else {
                    self.id[root_q as usize] = root_p;
                    self.rank[root_p as usize] += self.rank[root_q as usize];
                }

                self.count -= 1;
            }

            pub fn connected(&self, p: $num_type, q: $num_type) -> bool {
                self.find(q) == self.find(p)
            }
        }
        )*
    };
}

macro_rules! generate_qu_uf_impl {
    ($($num_type:ident), *) => {
        $(
        impl UnionFind<QuickUnion, $num_type> {
            pub fn find(&self, mut i: $num_type) -> $num_type {
                while i != self.id[i as usize] {
                    i = self.id[i as usize]
                }
                i
            }

            pub fn union(&mut self, p: $num_type, q: $num_type) {
                // find root of p and root of q and connect p to q
                // ie. making the root q the new root of p
                let root_p = self.find(p);
                let root_q = self.find(q);
                
                if root_p == root_q {
                    return;
                }

                self.count -= 1;
                self.id[root_p as usize] = root_q;
            }

            pub fn connected(&self, p: $num_type, q: $num_type) -> bool {
                self.find(q) == self.find(p)
            }
        }
        )*
    };
}

macro_rules! generate_uf_constructor_impl {
    ($($num_type:ident), *) => {
        $(
        impl <A> UnionFind<A, $num_type> {
            pub fn new(n: $num_type) -> Self {
                Self {
                    id: (0..n).collect(),
                    phantom: PhantomData::default(),
                    count: n as usize,
                    rank: PhantomData::default()
                }
            }
        }
        )*
    };
}

generate_uf_constructor_impl!(u8, i8, u16, i16, u32, i32, u64, i64, isize, usize);
generate_qf_uf_impl!(u8, i8, u16, i16, u32, i32, u64, i64, isize, usize);
generate_qu_uf_impl!(u8, i8, u16, i16, u32, i32, u64, i64, isize, usize);
generate_wqu_uf_impl!(u8, i8, u16, i16, u32, i32, u64, i64, isize, usize);

#[derive(Debug)]
pub struct QuickFind;

#[derive(Debug)]
pub struct QuickUnion;

#[derive(Debug)]
pub struct WeightedQuickUnion;

/// Weighted quick union with path compression
#[derive(Debug)]
pub struct WQupc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnionFind<A, T, RankContainer = PhantomData<()>> {
    pub id: Vec<T>,
    pub count: usize,
    pub rank: RankContainer,
    phantom: PhantomData<A>,
}

#[cfg(test)]
mod test {
    use crate::union_find::QuickFind;

    use super::{QuickUnion, UnionFind, WeightedQuickUnion};

    #[test]
    fn ui_test() {
        let uf = UnionFind::<QuickUnion, i32>::new(10);
        let uf = UnionFind::<QuickFind, i32>::new(10);
        let uf = UnionFind::<WeightedQuickUnion, i32, Vec<i32>>::new(10);
    }

    #[test]
    fn quick_find_test() {
        let mut qf = UnionFind::<QuickFind, i32>::new(10);
        qf.union(4, 3);
        dbg!(&qf);
        qf.union(3, 8);
        dbg!(&qf);
        qf.union(6, 5);
        dbg!(&qf);
        qf.union(9, 4);
        dbg!(&qf);
        dbg!(qf.connected(3, 9));
    }

    #[test]
    fn quick_union_test() {
        let mut qf = UnionFind::<QuickUnion, i32>::new(10);
        qf.union(4, 3);
        dbg!(&qf);
        qf.union(3, 8);
        dbg!(&qf);
        qf.union(6, 5);
        dbg!(&qf);
        qf.union(9, 4);
        dbg!(&qf);
        dbg!(qf.connected(3, 9));
    }

    #[test]
    fn weighted_quick_union_test() {
        let mut uf = UnionFind::<WeightedQuickUnion, usize, Vec<usize>>::new(10);
        uf.union(4, 3);
        uf.union(3, 8);
        uf.union(6, 5);
        uf.union(9, 4);
        uf.union(2, 1);
        uf.union(5, 0);
        assert_eq!(vec![6,2,2,4,4,6,6,7,4,4], uf.id);
    }
}
