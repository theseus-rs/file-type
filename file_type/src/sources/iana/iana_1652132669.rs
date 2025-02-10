use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1652132669: FileType = FileType {
    file_format: &FileFormat {
        id: 1_652_132_669,
        source_type: SourceType::Iana,
        name: "geoxacml+xml",
        extensions: &[],
        media_types: &["application/geoxacml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
