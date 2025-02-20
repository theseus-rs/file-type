use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
