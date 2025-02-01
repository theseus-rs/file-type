use crate::format::FileFormat;

pub(crate) const LINGUIST_51239111: FileFormat = FileFormat {
    id: 51_239_111,
    puid: "linguist/51239111",
    name: "OASv3-yaml",
    extensions: &["yaml", "yml"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
