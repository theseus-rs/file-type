use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_95: FileType = FileType {
    file_format: &FileFormat {
        id: 95,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Web Query",
        extensions: &["iqy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
