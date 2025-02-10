use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2787: FileType = FileType {
    file_format: &FileFormat {
        id: 2_787,
        source_type: SourceType::Pronom,
        name: "CorelDraw Drawing",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
