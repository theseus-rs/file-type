use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_352: FileType = FileType {
    file_format: &FileFormat {
        id: 352,
        source_type: SourceType::Linguist,
        name: "Smalltalk",
        extensions: &["cs", "st"],
        media_types: &["text/x-stsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
