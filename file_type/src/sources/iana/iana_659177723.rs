use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_659177723: FileType = FileType {
    file_format: &FileFormat {
        id: 659_177_723,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-user-profile-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-user-profile-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
