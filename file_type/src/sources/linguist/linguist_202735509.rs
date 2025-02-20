use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_202735509: FileType = FileType {
    file_format: &FileFormat {
        id: 202_735_509,
        source_type: SourceType::Linguist,
        name: "ObjectScript",
        extensions: &["cls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
