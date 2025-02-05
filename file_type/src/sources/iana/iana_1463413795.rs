use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1463413795: FileFormat = FileFormat {
    id: 1_463_413_795,
    source_type: SourceType::Iana,
    name: "vnd.futoin+json",
    extensions: &[],
    media_types: &["application/vnd.futoin+json"],
    signatures: &[],
    related_formats: &[],
};
