use crate::format::FileFormat;

pub(crate) const LINGUIST_297: FileFormat = FileFormat {
    id: 297,
    puid: "linguist/297",
    name: "Protocol Buffer",
    extensions: &["proto"],
    media_types: &["text/x-protobuf"],
    internal_signatures: &[],
    related_formats: &[],
};
