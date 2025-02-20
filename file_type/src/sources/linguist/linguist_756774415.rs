use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_756774415: FileType = FileType {
    file_format: &FileFormat {
        id: 756_774_415,
        source_type: SourceType::Linguist,
        name: "Pact",
        extensions: &["pact"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
