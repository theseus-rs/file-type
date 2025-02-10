use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_77: FileType = FileType {
    file_format: &FileFormat {
        id: 77,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel ODBC Query",
        extensions: &["dqy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
