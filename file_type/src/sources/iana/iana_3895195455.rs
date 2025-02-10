use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3895195455: FileType = FileType {
    file_format: &FileFormat {
        id: 3_895_195_455,
        source_type: SourceType::Iana,
        name: "vnd.afpc.modca-overlay",
        extensions: &[],
        media_types: &["application/vnd.afpc.modca-overlay"],
        signatures: &[],
        related_formats: &[],
    },
};
