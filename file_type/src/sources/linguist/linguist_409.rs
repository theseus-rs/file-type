use crate::format::FileFormat;

pub(crate) const LINGUIST_409: FileFormat = FileFormat {
    id: 409,
    puid: "linguist/409",
    name: "Yacc",
    extensions: &["y", "yacc", "yy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
