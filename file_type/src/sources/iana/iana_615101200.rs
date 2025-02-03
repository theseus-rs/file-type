use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_615101200: FileFormat = FileFormat {
    id: 615_101_200,
    source_type: SourceType::Iana,
    name: "vnd.veryant.thin",
    extensions: &[],
    media_types: &["application/vnd.veryant.thin"],
    internal_signatures: &[],
    related_formats: &[],
};
