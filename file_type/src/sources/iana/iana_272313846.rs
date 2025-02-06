use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_272313846: FileFormat = FileFormat {
    id: 272_313_846,
    source_type: SourceType::Iana,
    name: "vnd.ms-artgalry",
    extensions: &[],
    media_types: &["application/vnd.ms-artgalry"],
    signatures: &[],
    related_formats: &[],
};
