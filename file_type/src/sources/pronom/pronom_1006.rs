use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1006: FileType = FileType {
    file_format: &FileFormat {
        id: 1_006,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Database for Macintosh",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
