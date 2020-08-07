#[cfg(feature = "fnv")]
pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;

#[cfg(not(feature = "fnv"))]
pub type HashMap<K, V> = std::collections::HashMap<K, V>;