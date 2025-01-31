use crate::format::FileFormat;

pub(crate) const LINGUIST_77: FileFormat = FileFormat {
    id: 77,
    puid: "linguist/77",
    name: "Cuda",
    extensions: &["cu", "cuh"],
    media_types: &["text/x-c++src"],
    internal_signatures: &[],
    related_formats: &[],
};
