use crate::format::FileFormat;

pub(crate) const LINGUIST_211: FileFormat = FileFormat {
    id: 211,
    puid: "linguist/211",
    name: "LookML",
    extensions: &["lkml", "lookml"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
