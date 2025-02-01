use crate::format::FileFormat;

pub(crate) const LINGUIST_292: FileFormat = FileFormat {
    id: 292,
    puid: "linguist/292",
    name: "PowerBuilder",
    extensions: &["pbt", "sra", "sru", "srw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
