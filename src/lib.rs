#[macro_export]
macro_rules! take_for_scope {
    ( $option:ident, $type:ty ) => {
        {
            struct Value<T:Clone> {
                data:*const Option<T>,
                from:*mut Option<T>,
            }

            impl<T:Clone> Drop for Value<T> {
                fn drop(&mut self) {
                    unsafe {
                        *self.from = Some((*self.data).as_ref().unwrap().clone());
                    }
                }
            }
            let ret = $option.take();
            (ret, Value {
                data: &ret as *const Option<$type>,
                from: $option as *mut Option<$type>,
            })
        }
    };
}
