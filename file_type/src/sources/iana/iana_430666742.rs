use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_430666742: FileType = FileType {
    file_format: &FileFormat {
        id: 430_666_742,
        source_type: SourceType::Iana,
        name: "vnd.etsi.simservs+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.simservs+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
