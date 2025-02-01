use crate::format::FileFormat;

pub(crate) const LINGUIST_273: FileFormat = FileFormat {
    id: 273,
    puid: "linguist/273",
    name: "PLSQL",
    extensions: &[
        "bdy", "ddl", "fnc", "pck", "pkb", "pks", "plb", "pls", "plsql", "prc", "spc", "sql",
        "tpb", "tps", "trg", "vw",
    ],
    media_types: &["text/x-plsql"],
    internal_signatures: &[],
    related_formats: &[],
};
