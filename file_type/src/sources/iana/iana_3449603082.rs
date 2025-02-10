use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3449603082: FileType = FileType {
    file_format: &FileFormat {
        id: 3_449_603_082,
        source_type: SourceType::Iana,
        name: "vnd.fujixerox.ART-EX",
        extensions: &[],
        media_types: &["application/vnd.fujixerox.ART-EX"],
        signatures: &[],
        related_formats: &[],
    },
};
