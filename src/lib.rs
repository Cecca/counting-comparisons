use std::cell::Cell;
use std::sync::atomic::AtomicU64;

// --------------------------------------------------------------
thread_local! {
    static CELL_COUNTER: Cell<u64> = Cell::new(0);
}
pub fn get_cell_counter() -> u64 {
    CELL_COUNTER.with(|c| c.get())
}

#[derive(Eq)]
pub struct WrapCellThreadLocal(u64);
impl PartialEq for WrapCellThreadLocal {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Ord for WrapCellThreadLocal {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        CELL_COUNTER.with(|c| {
            c.set(c.get() + 1);
        });
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for WrapCellThreadLocal {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl From<u64> for WrapCellThreadLocal {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

// --------------------------------------------------------------
static ATOMIC_COUNTER: AtomicU64 = AtomicU64::new(0);
pub fn get_atomic_counter() -> u64 {
    ATOMIC_COUNTER.load(std::sync::atomic::Ordering::SeqCst)
}


#[derive(Eq)]
pub struct WrapAtomic(u64);
impl PartialEq for WrapAtomic {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Ord for WrapAtomic {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        ATOMIC_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for WrapAtomic {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl From<u64> for WrapAtomic {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

// --------------------------------------------------------------
static mut UNSAFE_CNT: u64 = 0;
pub fn get_unsafe_counter() -> u64 {
    unsafe {
        UNSAFE_CNT
    }
}

#[derive(Eq)]
pub struct WrapUnsafeCnt(u64);
impl PartialEq for WrapUnsafeCnt {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Ord for WrapUnsafeCnt {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            UNSAFE_CNT += 1;
        }
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for WrapUnsafeCnt {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl From<u64> for WrapUnsafeCnt {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    use rand::distributions::Standard;
    use rand::prelude::*;

    fn vec_from<T: From<u64>>(orig: &[u64]) -> Vec<T> {
        orig.iter().map(|x| T::from(*x)).collect()
    }

    #[test]
    fn check_size() {
        assert_eq!(size_of::<u64>(), size_of::<WrapAtomic>());
        assert_eq!(size_of::<u64>(), size_of::<WrapUnsafeCnt>());
        assert_eq!(size_of::<u64>(), size_of::<WrapCellThreadLocal>());
    }

    #[test]
    fn check_count() {
        let n = 10000;
        let rng = rand::thread_rng();
        let vals: Vec<u64> = rng.sample_iter(Standard).take(n).collect();

        vec_from::<WrapAtomic>(&vals).sort();
        vec_from::<WrapUnsafeCnt>(&vals).sort();
        vec_from::<WrapCellThreadLocal>(&vals).sort();

        assert_eq!(get_cell_counter(), get_atomic_counter());
        assert_eq!(get_cell_counter(), get_unsafe_counter());
    }
}

