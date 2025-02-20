use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1386: FileType = FileType {
    file_format: &FileFormat {
        id: 1_386,
        source_type: SourceType::Pronom,
        name: "Microsoft PhotoDraw",
        extensions: &["mix"],
        media_types: &["image/vnd.mix"],
        signatures: &[],
        related_formats: &[],
    },
};
