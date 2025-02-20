use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
