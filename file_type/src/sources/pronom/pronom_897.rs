use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
