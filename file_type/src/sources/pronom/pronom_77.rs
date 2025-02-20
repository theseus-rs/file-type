use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
