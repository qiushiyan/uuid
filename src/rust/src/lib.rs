use derive_more::From;
use extendr_api::prelude::*;
use rayon::prelude::*;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

#[derive(Clone, Debug, From)]
struct Id(Uuid);

enum Version {
    V1,
    V3,
    V4,
    V5,
}

impl From<u8> for Version {
    fn from(v: u8) -> Self {
        match v {
            1 => Version::V1,
            3 => Version::V3,
            4 => Version::V4,
            5 => Version::V5,
            _ => panic!("Invalid version"),
        }
    }
}

impl Id {
    fn new(version: u8) -> Self {
        let v: Version = version.into();
        match v {
            Version::V4 => Id(Uuid::new_v4()),
            Version::V5 => Id(Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"rust-lang.org")),
            Version::V3 => Id(Uuid::new_v3(&Uuid::NAMESPACE_DNS, b"rust-lang.org")),
            Version::V1 => {
                let context = Context::new(42);
                let ts = Timestamp::from_unix(&context, 1497624119, 1234);
                Id(Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]))
            }
        }
    }

    fn into_inner(self) -> String {
        self.0.to_string()
    }
}

/// Generate uuid
/// @export
#[extendr]
fn uuid_(n: u64, version: u8) -> Vec<String> {
    let size: usize = n.try_into().unwrap();
    let mut ids: Vec<String> = vec![String::from(""); size];
    ids.par_iter_mut()
        .for_each(|id| *id = Id::new(version).into_inner());

    ids
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod uuid;
    fn uuid_;
}
