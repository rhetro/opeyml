// ============================================================================
// opeyml: The Absolute Surgical DSL for YAML
// ============================================================================

// ============================================================================
// 1. STRICT MODE: Precision Surgery
// ============================================================================

// -----------------------------------------------------------------------------
// 1.1. BIOPSY (Strict Read)
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_biopsy {
    ($val:expr) => { Some($val) };
    ($val:expr, ) => { Some($val) };
    ($val:expr, . ($key:expr) $($rest:tt)*) => {
        match $val.get($key) {
            Some(inner) => $crate::__impl_yml_biopsy!(inner, $($rest)*),
            None => None,
        }
    };
    ($val:expr, . $key:literal $($rest:tt)*) => {
        match $val.get($key) {
            Some(inner) => $crate::__impl_yml_biopsy!(inner, $($rest)*),
            None => None,
        }
    };
    ($val:expr, . $key:ident $($rest:tt)*) => {
        match $val.get(std::stringify!($key)) {
            Some(inner) => $crate::__impl_yml_biopsy!(inner, $($rest)*),
            None => None,
        }
    };
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {
        match $val.get($idx as usize) {
            Some(inner) => $crate::__impl_yml_biopsy!(inner, $($rest)*),
            None => None,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _biopsy {
    ($val:expr, $($args:tt)+) => {{
        let _: &$crate::serde_yaml::Value = &$val;
        $crate::__impl_yml_biopsy!($val, $($args)+)
    }};
    ($val:expr) => {{
        let _: &$crate::serde_yaml::Value = &$val;
        Some($val)
    }};
}

// -----------------------------------------------------------------------------
// 1.2. INCISE (Strict Write)
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_incise {
    ($val:expr, = [ $($item:tt)* ]) => {{ *$val = $crate::__impl_yml_forge!([ $($item)* ]); Some(()) }};
    ($val:expr, = { $($item:tt)* }) => {{ *$val = $crate::__impl_yml_forge!({ $($item)* }); Some(()) }};
    ($val:expr, = $value:expr) => {{ *$val = $crate::serde_yaml::Value::from($value); Some(()) }};
    ($val:expr) => { Some($val) };
    ($val:expr, . ($key:expr) $($rest:tt)*) => {
        match $val.get_mut($key) {
            Some(inner) => $crate::__impl_yml_incise!(inner, $($rest)*),
            None => None,
        }
    };
    ($val:expr, . $key:literal $($rest:tt)*) => {
        match $val.get_mut($key) {
            Some(inner) => $crate::__impl_yml_incise!(inner, $($rest)*),
            None => None,
        }
    };
    ($val:expr, . $key:ident $($rest:tt)*) => {
        match $val.get_mut(std::stringify!($key)) {
            Some(inner) => $crate::__impl_yml_incise!(inner, $($rest)*),
            None => None,
        }
    };
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {
        match $val.get_mut($idx as usize) {
            Some(inner) => $crate::__impl_yml_incise!(inner, $($rest)*),
            None => None,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _incise {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_yaml::Value = &mut $val;
        $crate::__impl_yml_incise!((&mut $val), $($args)+)
    }};
    ($val:expr) => {{
        let _: &mut $crate::serde_yaml::Value = &mut $val;
        Some($val)
    }};
}

// -----------------------------------------------------------------------------
// 1.3. EXCISE (Delete)
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_excise {
    ($val:expr, . ($target:expr)) => {
        $val.as_mapping_mut().and_then(|m| m.remove(&$crate::serde_yaml::Value::from($target)))
    };
    ($val:expr, . $target:literal) => {
        $val.as_mapping_mut().and_then(|m| m.remove(&$crate::serde_yaml::Value::from($target)))
    };
    ($val:expr, . $target:ident) => {
        $val.as_mapping_mut().and_then(|m| m.remove(&$crate::serde_yaml::Value::String(std::stringify!($target).to_string())))
    };
    ($val:expr, [ $idx:expr ]) => {
        $val.as_sequence_mut().and_then(|a| {
            let idx = $idx as usize;
            if idx < a.len() { Some(a.remove(idx)) } else { None }
        })
    };
    ($val:expr, . ($key:expr) $($rest:tt)+) => {
        match $val.get_mut($key) {
            Some(v) => $crate::__impl_yml_excise!(v, $($rest)+),
            None => None,
        }
    };
    ($val:expr, . $key:literal $($rest:tt)+) => {
        match $val.get_mut($key) {
            Some(v) => $crate::__impl_yml_excise!(v, $($rest)+),
            None => None,
        }
    };
    ($val:expr, . $key:ident $($rest:tt)+) => {
        match $val.get_mut(std::stringify!($key)) {
            Some(v) => $crate::__impl_yml_excise!(v, $($rest)+),
            None => None,
        }
    };
    ($val:expr, [ $idx:expr ] $($rest:tt)+) => {
        match $val.get_mut($idx as usize) {
            Some(v) => $crate::__impl_yml_excise!(v, $($rest)+),
            None => None,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _excise {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_yaml::Value = &mut $val;
        $crate::__impl_yml_excise!($val, $($args)+)
    }};
}

// ============================================================================
// 2. GENESIS MODE: Structure Creation & Manipulation
// ============================================================================

// -----------------------------------------------------------------------------
// 2.0. THE MATTER FORGE
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_forge_count {
    () => { 0usize };
    ($_:tt $(, $rest:tt)* $(,)?) => { 1usize + $crate::__impl_yml_forge_count!($($rest)*) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_forge {
    ([ ]) => { $crate::serde_yaml::Value::Sequence(std::vec::Vec::new()) };
    ([ $($item:tt),+ $(,)? ]) => {{
        let cap = $crate::__impl_yml_forge_count!($($item)*);
        let mut v = std::vec::Vec::with_capacity(cap);
        $( v.push($crate::__impl_yml_forge!($item)); )*
        $crate::serde_yaml::Value::Sequence(v)
    }};
    ({ }) => { $crate::serde_yaml::Value::Mapping($crate::serde_yaml::Mapping::new()) };
    ({ $($key:tt : $val:tt),+ $(,)? }) => {{
        let mut m = $crate::serde_yaml::Mapping::new();
        $(
            m.insert(
                $crate::serde_yaml::Value::from($key),
                $crate::__impl_yml_forge!($val)
            );
        )*
        $crate::serde_yaml::Value::Mapping(m)
    }};
    ($val:expr) => { $crate::serde_yaml::Value::from($val) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __opeyml_key {
    (($e:expr)) => {
        $crate::serde_yaml::Value::from($e)
    };
    ($l:literal) => {
        $crate::serde_yaml::Value::from($l)
    };
    ($i:ident) => {
        $crate::serde_yaml::Value::String(std::stringify!($i).to_string())
    };
}

// -----------------------------------------------------------------------------
// 2.1. SUTURE
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_suture {
    ($val:expr, = [ $($item:tt)* ]) => { *$val = $crate::__impl_yml_forge!([ $($item)* ]); };
    ($val:expr, = { $($item:tt)* }) => { *$val = $crate::__impl_yml_forge!({ $($item)* }); };
    ($val:expr, = $value:expr) => { *$val = $crate::serde_yaml::Value::from($value); };

    ($val:expr, . $key:tt $($rest:tt)*) => {{
        if $val.is_null() { *$val = $crate::__impl_yml_forge!({ }); }
        if let $crate::serde_yaml::Value::Mapping(map) = $val {
            let next = map.entry($crate::__opeyml_key!($key))
                          .or_insert($crate::serde_yaml::Value::Null);
            $crate::__impl_yml_suture!(next, $($rest)*);
        }
    }};
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {{
        if $val.is_null() { *$val = $crate::__impl_yml_forge!([ ]); }
        if let $crate::serde_yaml::Value::Sequence(arr) = $val {
            let idx = $idx as usize;
            if idx >= arr.len() { arr.resize(idx + 1, $crate::serde_yaml::Value::Null); }
            $crate::__impl_yml_suture!(&mut arr[idx], $($rest)*);
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _suture {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_yaml::Value = &mut $val;
        $crate::__impl_yml_suture!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 2.2. FORCE SUTURE
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_force_suture {
    ($val:expr, = [ $($item:tt)* ]) => { *$val = $crate::__impl_yml_forge!([ $($item)* ]); };
    ($val:expr, = { $($item:tt)* }) => { *$val = $crate::__impl_yml_forge!({ $($item)* }); };
    ($val:expr, = $value:expr) => { *$val = $crate::serde_yaml::Value::from($value); };

    ($val:expr, . $key:tt $($rest:tt)*) => {{
        if !$val.is_mapping() { *$val = $crate::__impl_yml_forge!({ }); }
        if let $crate::serde_yaml::Value::Mapping(map) = $val {
            let next = map.entry($crate::__opeyml_key!($key))
                          .or_insert($crate::serde_yaml::Value::Null);
            $crate::__impl_yml_force_suture!(next, $($rest)*);
        }
    }};
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {{
        if !$val.is_sequence() { *$val = $crate::__impl_yml_forge!([ ]); }
        if let $crate::serde_yaml::Value::Sequence(arr) = $val {
            let idx = $idx as usize;
            if idx >= arr.len() { arr.resize(idx + 1, $crate::serde_yaml::Value::Null); }
            $crate::__impl_yml_force_suture!(&mut arr[idx], $($rest)*);
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _force_suture {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_yaml::Value = &mut $val;
        $crate::__impl_yml_force_suture!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 2.3. ACQUIRE
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! _acquire {
    ($val:expr, $($path:tt)+) => {{
        $crate::_biopsy!($val, $($path)+)
            .ok_or_else(|| $crate::Error::PathNotFound(format!("{}", std::stringify!($($path)+))))
    }};
}

// -----------------------------------------------------------------------------
// 2.4. IMPLANT
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_implant {
    ($val:expr, = [ $($item:tt)* ]) => { if $val.is_null() { *$val = $crate::__impl_yml_forge!([ $($item)* ]); } };
    ($val:expr, = { $($item:tt)* }) => { if $val.is_null() { *$val = $crate::__impl_yml_forge!({ $($item)* }); } };
    ($val:expr, = $value:expr) => { if $val.is_null() { *$val = $crate::serde_yaml::Value::from($value); } };
    ($val:expr, . $key:tt $($rest:tt)*) => {{
        if $val.is_null() { *$val = $crate::__impl_yml_forge!({ }); }
        if let $crate::serde_yaml::Value::Mapping(map) = $val {
            let next = map.entry($crate::__opeyml_key!($key))
                          .or_insert($crate::serde_yaml::Value::Null);
            $crate::__impl_yml_implant!(next, $($rest)*);
        }
    }};
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {{
        if $val.is_null() { *$val = $crate::__impl_yml_forge!([ ]); }
        if let $crate::serde_yaml::Value::Sequence(arr) = $val {
            let idx = $idx as usize;
            if idx >= arr.len() { arr.resize(idx + 1, $crate::serde_yaml::Value::Null); }
            $crate::__impl_yml_implant!(&mut arr[idx], $($rest)*);
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _implant {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_yaml::Value = &mut $val;
        $crate::__impl_yml_implant!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 2.5. GRAFT
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_graft {
    ($val:expr, = [ $($item:tt)* ]) => {{
        match (&mut *$val, $crate::__impl_yml_forge!([ $($item)* ])) {
            ($crate::serde_yaml::Value::Sequence(h_seq), $crate::serde_yaml::Value::Sequence(mut s_seq)) => { h_seq.append(&mut s_seq); }
            (h_val, s_val) => { *h_val = s_val; }
        }
    }};
    ($val:expr, = { $($item:tt)* }) => {{
        match (&mut *$val, $crate::__impl_yml_forge!({ $($item)* })) {
            ($crate::serde_yaml::Value::Mapping(h_map), $crate::serde_yaml::Value::Mapping(s_map)) => { h_map.extend(s_map); }
            (h_val, s_val) => { *h_val = s_val; }
        }
    }};
    ($val:expr, = $value:expr) => {{
        match (&mut *$val, $crate::serde_yaml::Value::from($value)) {
            ($crate::serde_yaml::Value::Mapping(h_map), $crate::serde_yaml::Value::Mapping(s_map)) => { h_map.extend(s_map); }
            ($crate::serde_yaml::Value::Sequence(h_seq), $crate::serde_yaml::Value::Sequence(mut s_seq)) => { h_seq.append(&mut s_seq); }
            (h_val, s_val) => { *h_val = s_val; }
        }
    }};
    ($val:expr, . $key:tt $($rest:tt)*) => {{
        if $val.is_null() { *$val = $crate::__impl_yml_forge!({ }); }
        if let $crate::serde_yaml::Value::Mapping(map) = $val {
            let next = map.entry($crate::__opeyml_key!($key))
                          .or_insert($crate::serde_yaml::Value::Null);
            $crate::__impl_yml_graft!(next, $($rest)*);
        }
    }};
    ($val:expr, [ $idx:expr ] $($rest:tt)*) => {{
        if $val.is_null() { *$val = $crate::__impl_yml_forge!([ ]); }
        if let $crate::serde_yaml::Value::Sequence(arr) = $val {
            let idx = $idx as usize;
            if idx >= arr.len() { arr.resize(idx + 1, $crate::serde_yaml::Value::Null); }
            $crate::__impl_yml_graft!(&mut arr[idx], $($rest)*);
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _graft {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_yaml::Value = &mut $val;
        $crate::__impl_yml_graft!((&mut $val), $($args)+);
    }};
}

// -----------------------------------------------------------------------------
// 2.6. MESH
// -----------------------------------------------------------------------------
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_yml_construct_room {
    ([ $len:expr ] , $init:expr) => {{
        let len = $len as usize;
        let init_val = $crate::serde_yaml::Value::from($init);
        let mut v = std::vec::Vec::with_capacity(len);
        for _ in 0..len { v.push(init_val.clone()); }
        $crate::serde_yaml::Value::Sequence(v)
    }};
    ([ $len:expr ]) => {
        $crate::__impl_yml_construct_room!([ $len ] , $crate::serde_yaml::Value::Null)
    };
    ([ $len:expr ] [ $($next:tt)+ ] $($rest:tt)*) => {{
        let len = $len as usize;
        let mut v = std::vec::Vec::with_capacity(len);
        for _ in 0..len {
            v.push($crate::__impl_yml_construct_room!([ $($next)+ ] $($rest)*));
        }
        $crate::serde_yaml::Value::Sequence(v)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _mesh {
    ($val:expr, $($args:tt)+) => {{
        let _: &mut $crate::serde_yaml::Value = &mut $val;
        $val = $crate::__impl_yml_construct_room!($($args)+);
    }};
}
