// SPDX-License-Identifier: GPL-3.0-or-later
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher as Argon2PasswordHasher, PasswordVerifier, Version};
use pyo3::prelude::*;
use rand_core::OsRng;

#[pyclass]
#[derive(Clone, Copy)]
enum Type {
    ID,
    D,
    I,
}

impl Default for Type {
    fn default() -> Self {
        Self::ID
    }
}

impl From<Type> for Algorithm {
    fn from(value: Type) -> Self {
        match value {
            Type::ID => Algorithm::Argon2id,
            Type::D => Algorithm::Argon2d,
            Type::I => Algorithm::Argon2i,
        }
    }
}

#[pyclass]
struct PasswordHasher {
    algo: Algorithm,
    argon2: Argon2<'static>,
}

#[pymethods]
impl PasswordHasher {
    #[new]
    fn new(
        time_cost: Option<u32>,
        memory_cost: Option<u32>,
        parallelism: Option<u32>,
        hash_len: Option<usize>,
        //salt_len: Option<usize>,
        type_: Option<Type>,
    ) -> Self {
        let params = Params::new(
            memory_cost.unwrap_or(Params::DEFAULT_M_COST),
            time_cost.unwrap_or(Params::DEFAULT_T_COST),
            parallelism.unwrap_or(Params::DEFAULT_P_COST),
            hash_len,
        )
        .unwrap();
        let algo = type_.unwrap_or_default().into();
        let argon2 = Argon2::new(
            algo,
            Version::default(),
            params,
        );
        PasswordHasher { algo, argon2 }
    }

    pub fn hash(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }

    pub fn verify(&self, hash: &str, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).unwrap();
        self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }

    pub fn check_needs_rehash(&self, hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).unwrap();
        if parsed_hash.algorithm != self.algo.ident() {
            return true;
        }
        if parsed_hash.params != self.argon2.params().try_into().unwrap() {
            return true;
        }
        false
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn argon2_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PasswordHasher>()?;
    m.add_class::<Type>()?;
    Ok(())
}
