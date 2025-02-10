use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_897: FileType = FileType {
    file_format: &FileFormat {
        id: 897,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Database for DOS",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
