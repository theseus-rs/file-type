use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_361: FileType = FileType {
    file_format: &FileFormat {
        id: 361,
        source_type: SourceType::Pronom,
        name: "Microsoft Project",
        extensions: &["mpp"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[],
    },
};
