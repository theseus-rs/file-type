use crate::format::FileFormat;

pub(crate) const LINGUIST_142: FileFormat = FileFormat {
    id: 142,
    puid: "linguist/142",
    name: "Groovy",
    extensions: &["groovy", "grt", "gtpl", "gvy"],
    media_types: &["text/x-groovy"],
    internal_signatures: &[],
    related_formats: &[],
};
