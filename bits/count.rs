use crate::prelude::*;

#[inline]
pub fn count1<T: ?Sized + Count>(t: &T) -> usize {
    t.count1()
}

#[inline]
pub fn count0<T: ?Sized + Count>(t: &T) -> usize {
    t.count0()
}

#[inline]
pub fn all<T: ?Sized + Count>(t: &T) -> bool {
    t.all()
}

#[inline]
pub fn any<T: ?Sized + Count>(t: &T) -> bool {
    t.any()
}

/// ## Implementing `Count`
///
/// Note that `count1` and `count0` are circularly referenced.
/// So, you need to implement at least **one** of them.
pub trait Count: Bits {
    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// let v: &[u64] = &[];
    /// let w: &[u64] = &[0, 0, 0];
    /// let x: &[u64] = &[0, 1, 3];
    /// assert_eq!(bits::count1(v), 0);
    /// assert_eq!(bits::count1(w), 0);
    /// assert_eq!(bits::count1(x), 3);
    /// ```
    #[inline]
    fn count1(&self) -> usize {
        Bits::len(self) - self.count0()
    }

    /// Counts the occurrences of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let v: &[u64] = &[];
    /// let w: &[u64] = &[0, 0, 0];
    /// let x: &[u64] = &[0, 1, 3];
    /// assert_eq!(v.count0(), 0);
    /// assert_eq!(w.count0(), 192);
    /// assert_eq!(x.count0(), 189);
    /// ```
    #[inline]
    fn count0(&self) -> usize {
        Bits::len(self) - self.count1()
    }

    /// Returns true if all bits are enabled. An empty bits should return true.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let v: &[u64] = &[0, 0, 0];
    /// let w: &[u64] = &[];
    /// let x: &[u64] = &[!0, !0, !0];
    /// assert!(!v.all());
    /// assert!( w.all());
    /// assert!( x.all());
    /// ```
    #[inline]
    fn all(&self) -> bool {
        Bits::is_empty(self) || self.count0() == 0
    }

    /// Returns true if any bits are enabled. An empty bits should return false.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Count;
    /// let v: &[u64] = &[0, 0, 0];
    /// let w: &[u64] = &[];
    /// let x: &[u64] = &[!0, !0, !0];
    /// let y: &[u64] = &[0, 0, 1];
    /// assert!(!v.any());
    /// assert!(!w.any());
    /// assert!( x.any());
    /// assert!( y.any());
    /// ```
    #[inline]
    fn any(&self) -> bool {
        !Bits::is_empty(self) && self.count1() > 0
    }
}

// pub trait All: Count {
//     fn all(&self) -> bool;
// }
// pub trait Any: Count {
//     fn any(&self) -> bool;
// }
//
// impl<T: ?Sized + Count> All for T {
//     #[inline]
//     default fn all(&self) -> bool {
//         Bits::is_empty(self) || self.count0() == 0
//     }
// }
// impl<T: ?Sized + Count> Any for T {
//     #[inline]
//     default fn any(&self) -> bool {
//         !Bits::is_empty(self) && self.count1() > 0
//     }
// }
