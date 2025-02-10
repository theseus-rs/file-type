use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2554140704: FileType = FileType {
    file_format: &FileFormat {
        id: 2_554_140_704,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.bsf+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.bsf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
