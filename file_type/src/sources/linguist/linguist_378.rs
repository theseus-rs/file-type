use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_378: FileType = FileType {
    file_format: &FileFormat {
        id: 378,
        source_type: SourceType::Linguist,
        name: "TypeScript",
        extensions: &["cts", "mts", "ts"],
        media_types: &["application/typescript"],
        signatures: &[],
        related_formats: &[],
    },
};
