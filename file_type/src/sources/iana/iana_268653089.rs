use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_268653089: FileFormat = FileFormat {
    id: 268_653_089,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.text-web",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.text-web"],
    signatures: &[],
    related_formats: &[],
};
