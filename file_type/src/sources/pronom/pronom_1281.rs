use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1281: FileType = FileType {
    file_format: &FileFormat {
        id: 1_281,
        source_type: SourceType::Pronom,
        name: "Microsoft Office Encrypted Document",
        extensions: &["xlsx", "pptx", "docx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
