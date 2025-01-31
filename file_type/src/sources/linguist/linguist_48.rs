use crate::format::FileFormat;

pub(crate) const LINGUIST_48: FileFormat = FileFormat {
    id: 48,
    puid: "linguist/48",
    name: "COBOL",
    extensions: &["cbl", "ccp", "cob", "cobol", "cpy"],
    media_types: &["text/x-cobol"],
    internal_signatures: &[],
    related_formats: &[],
};
