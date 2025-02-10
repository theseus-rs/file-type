use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2777984863: FileType = FileType {
    file_format: &FileFormat {
        id: 2_777_984_863,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.vae-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.vae-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
