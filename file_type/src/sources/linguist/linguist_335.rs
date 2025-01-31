use crate::format::FileFormat;

pub(crate) const LINGUIST_335: FileFormat = FileFormat {
    id: 335,
    puid: "linguist/335",
    name: "SRecode Template",
    extensions: &["srt"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
