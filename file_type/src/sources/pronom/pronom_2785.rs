use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2785: FileType = FileType {
    file_format: &FileFormat {
        id: 2_785,
        source_type: SourceType::Pronom,
        name: "CorelDraw Drawing",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
