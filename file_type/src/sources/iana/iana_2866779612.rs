use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2866779612: FileFormat = FileFormat {
    id: 2_866_779_612,
    source_type: SourceType::Iana,
    name: "index.vnd",
    extensions: &[],
    media_types: &["application/index.vnd"],
    signatures: &[],
    related_formats: &[],
};
