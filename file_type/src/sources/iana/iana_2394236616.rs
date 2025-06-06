use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2394236616: FileType = FileType {
    file_format: &FileFormat {
        id: 2_394_236_616,
        source_type: SourceType::Iana,
        name: "vnd.sealed.ppt",
        extensions: &[],
        media_types: &["application/vnd.sealed.ppt"],
        signatures: &[],
        related_formats: &[],
    },
};
