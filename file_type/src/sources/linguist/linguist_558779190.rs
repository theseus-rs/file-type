use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_558779190: FileType = FileType {
    file_format: &FileFormat {
        id: 558_779_190,
        source_type: SourceType::Linguist,
        name: "Sweave",
        extensions: &["rnw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
