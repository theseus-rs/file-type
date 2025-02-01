use crate::format::FileFormat;

pub(crate) const LINGUIST_208: FileFormat = FileFormat {
    id: 208,
    puid: "linguist/208",
    name: "LiveScript",
    extensions: &["_ls", "ls"],
    media_types: &["text/x-livescript"],
    internal_signatures: &[],
    related_formats: &[],
};
