#[macro_export]
macro_rules! define_id {
    ($id_type: ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $id_type(uuid::Uuid);

        impl $crate::shared::id::id_type::Id for $id_type {
            fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            fn raw(self) -> uuid::Uuid {
                self.0
            }
        }

        impl From<uuid::Uuid> for $id_type {
            fn from(u: uuid::Uuid) -> Self {
                Self(u)
            }
        }

        impl std::fmt::Display for $id_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

pub mod id_type {
    use uuid::Uuid;

    pub trait Id: Sized + Clone + std::fmt::Debug + PartialEq + Eq + Copy + From<Uuid> {
        fn new() -> Self;
        fn raw(self) -> uuid::Uuid;
    }
}

pub use id_type::Id;
