use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2437446151: FileType = FileType {
    file_format: &FileFormat {
        id: 2_437_446_151,
        source_type: SourceType::Iana,
        name: "geoxacml+json",
        extensions: &[],
        media_types: &["application/geoxacml+json"],
        signatures: &[],
        related_formats: &[],
    },
};
