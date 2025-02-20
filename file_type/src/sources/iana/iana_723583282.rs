use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_723583282: FileType = FileType {
    file_format: &FileFormat {
        id: 723_583_282,
        source_type: SourceType::Iana,
        name: "cdmi-object",
        extensions: &[],
        media_types: &["application/cdmi-object"],
        signatures: &[],
        related_formats: &[],
    },
};
