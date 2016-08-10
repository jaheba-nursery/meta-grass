

#[macro_export]
macro_rules! create_bind {
    ($target:ident for $core:ty where $($x:ident : $t:ty),+) => {

        pub struct $target<'a, 'core:'a> {
            pub _core: &'a mut $core,
            $(pub $x: &'a mut $t),*
        }

        impl<'a, 'core> Deref for $target<'a, 'core> {
            type Target = $core;

            fn deref(&self) -> &$core {
                &self._core
            }
        }

        impl<'a, 'core> DerefMut for $target<'a, 'core> {
            fn deref_mut(&mut self) -> &mut $core {
                &mut self._core
            }
        }

        // make `bind_mut` available on source type
        impl<'core> $core {
            pub fn bind_mut<'a>(&'a mut self, $($x: &'a mut $t),*) -> $target<'a, 'core> {
                $target { _core: self, $($x: $x),* }
            }
        }

    }
}

use std::cmp::PartialEq;

pub trait MergePoint: PartialEq {
}


pub fn merge_point<T : MergePoint>(mp: &mut T) {

}
