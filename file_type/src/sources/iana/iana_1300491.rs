use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1300491: FileType = FileType {
    file_format: &FileFormat {
        id: 1_300_491,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcdata-payload",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcdata-payload"],
        signatures: &[],
        related_formats: &[],
    },
};
