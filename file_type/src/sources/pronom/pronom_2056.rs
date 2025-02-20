use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2056: FileType = FileType {
    file_format: &FileFormat {
        id: 2_056,
        source_type: SourceType::Pronom,
        name: "Band Interleaved By Line (BIL) Image Encoding",
        extensions: &["bil"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
