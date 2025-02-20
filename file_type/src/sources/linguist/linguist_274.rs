use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_274: FileType = FileType {
    file_format: &FileFormat {
        id: 274,
        source_type: SourceType::Linguist,
        name: "PLpgSQL",
        extensions: &["pgsql", "sql"],
        media_types: &["text/x-sql"],
        signatures: &[],
        related_formats: &[],
    },
};
