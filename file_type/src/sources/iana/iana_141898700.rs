use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_141898700: FileType = FileType {
    file_format: &FileFormat {
        id: 141_898_700,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.crs+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.crs+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
