use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_341: FileType = FileType {
    file_format: &FileFormat {
        id: 341,
        source_type: SourceType::Linguist,
        name: "Scala",
        extensions: &["kojo", "sbt", "sc", "scala"],
        media_types: &["text/x-scala"],
        signatures: &[],
        related_formats: &[],
    },
};
