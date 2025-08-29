pub mod helper;
pub mod select_eval;
pub type LgResult<T, E = crate::error_lib::evaluation::EvalEror<String>> =
    std::result::Result<T, E>;

/*TODO :
    1)ALIAS DE COLUMN :
CRÉER UN HASMAP  type AliasMap = HashMap<String, String>; // ex: "u" -> "users";
    2)GESTION DES ORIGINES DE TABLE :
???
    3)GESTION DES ALIAS DE TABLE :
???
*/
