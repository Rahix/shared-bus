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

#[cfg(feature = "cortex-m")]
#[macro_export]
macro_rules! new_cortexm {
    ($bus_type:ty = $bus:expr) => {{
        let m: Option<&'static mut _> = $crate::cortex_m::singleton!(
            : $crate::BusManagerCortexM<$bus_type> =
                $crate::BusManagerCortexM::new($bus)
        );

        m
    }};
}
