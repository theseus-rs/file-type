use crate::format::FileFormat;

pub(crate) const LINGUIST_309: FileFormat = FileFormat {
    id: 309,
    puid: "linguist/309",
    name: "RDoc",
    extensions: &["rdoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
