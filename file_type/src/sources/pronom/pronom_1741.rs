use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1741: FileType = FileType {
    file_format: &FileFormat {
        id: 1_741,
        source_type: SourceType::Pronom,
        name: "Microsoft Picture It! Image File",
        extensions: &["mix"],
        media_types: &["image/vnd.mix"],
        signatures: &[],
        related_formats: &[],
    },
};
