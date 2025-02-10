use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3498786448: FileType = FileType {
    file_format: &FileFormat {
        id: 3_498_786_448,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcdata-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcdata-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
