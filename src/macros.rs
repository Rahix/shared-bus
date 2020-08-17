#[cfg(feature = "std")]
#[macro_export]
macro_rules! new_std {
    ($bus_type:ty = $bus:expr) => {{
        use $crate::once_cell::sync::OnceCell;

        static MANAGER: OnceCell<$crate::BusManagerStd<$bus_type>> = OnceCell::new();

        let m = $crate::BusManagerStd::new($bus);
        match MANAGER.set(m) {
            Ok(_) => MANAGER.get(),
            Err(_) => None,
        }
    }};
}
