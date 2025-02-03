use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_43235474: FileFormat = FileFormat {
    id: 43_235_474,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.graphics-template",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.graphics-template"],
    internal_signatures: &[],
    related_formats: &[],
};
