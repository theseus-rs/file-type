use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_999: FileType = FileType {
    file_format: &FileFormat {
        id: 999,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Database for DOS",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
