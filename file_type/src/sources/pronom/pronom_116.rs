use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_116: FileType = FileType {
    file_format: &FileFormat {
        id: 116,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel OLAP Query",
        extensions: &["oqy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
