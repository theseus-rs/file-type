use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3905661247: FileFormat = FileFormat {
    id: 3_905_661_247,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.graphics",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.graphics"],
    signatures: &[],
    related_formats: &[],
};
