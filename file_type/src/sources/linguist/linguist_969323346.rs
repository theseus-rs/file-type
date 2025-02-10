use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_969323346: FileType = FileType {
    file_format: &FileFormat {
        id: 969_323_346,
        source_type: SourceType::Linguist,
        name: "Dafny",
        extensions: &["dfy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
