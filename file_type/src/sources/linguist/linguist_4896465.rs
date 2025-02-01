use crate::format::FileFormat;

pub(crate) const LINGUIST_4896465: FileFormat = FileFormat {
    id: 4_896_465,
    puid: "linguist/4896465",
    name: "MiniYAML",
    extensions: &["yaml", "yml"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
