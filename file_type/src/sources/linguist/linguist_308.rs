use crate::format::FileFormat;

pub(crate) const LINGUIST_308: FileFormat = FileFormat {
    id: 308,
    puid: "linguist/308",
    name: "RAML",
    extensions: &["raml"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
