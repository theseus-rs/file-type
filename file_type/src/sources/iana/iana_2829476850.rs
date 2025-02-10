use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2829476850: FileType = FileType {
    file_format: &FileFormat {
        id: 2_829_476_850,
        source_type: SourceType::Iana,
        name: "vnd.si.simp (OBSOLETED by request)",
        extensions: &[],
        media_types: &["message/vnd.si.simp"],
        signatures: &[],
        related_formats: &[],
    },
};
