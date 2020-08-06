#[macro_export]
macro_rules! block {
    (($v:literal = var $w:literal )) => {
        $crate::cfg::Statement::VarAssign(
            $crate::cfg::Variable($v), 
            $crate::cfg::Variable($w)
        )
    };

    (($v:literal = $w:literal)) => {
        $crate::cfg::Statement::ConstAssign($crate::cfg::Variable($v), $w)
    };

    // 0; from => ; to => 1, 2; 0 = 5; 1 = var 0
    { $id:literal ; from => $( $from:literal ),* ; to => $( $to:literal ),* ; $( $s:tt );* } => {
        $crate::cfg::Block {
            id: $crate::cfg::BlockId($id),
            stmts: vec![$( block!($s) ),*],
            preds: vec![$( $crate::cfg::BlockId($from) ),*],
            succs: vec![$( $crate::cfg::BlockId($to) ),*]
        }
    };
}

#[macro_export]
macro_rules! set {
    [] => {
        fnv::FnvHashSet::default()
    };

    [$( $v:expr ),+] => {{
        let mut res = fnv::FnvHashSet::default();
        $( res.insert($v) );*;
        res
    }}
}

#[macro_export]
macro_rules! dict {
    [] => {
        fnv::FnvHashMap::default()
    };

    [$( $k:expr => $v:expr ),+] => {{
        let mut res = fnv::FnvHashMap::default();
        $( res.insert($k, $v) );*;
        res
    }}
}