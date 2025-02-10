use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2403512592: FileType = FileType {
    file_format: &FileFormat {
        id: 2_403_512_592,
        source_type: SourceType::Iana,
        name: "automationml-aml+xml",
        extensions: &[],
        media_types: &["application/automationml-aml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
