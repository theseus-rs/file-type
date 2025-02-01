use crate::format::FileFormat;

pub(crate) const LINGUIST_230: FileFormat = FileFormat {
    id: 230,
    puid: "linguist/230",
    name: "Metal",
    extensions: &["metal"],
    media_types: &["text/x-c++src"],
    internal_signatures: &[],
    related_formats: &[],
};
