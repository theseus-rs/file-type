use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_173616037: FileType = FileType {
    file_format: &FileFormat {
        id: 173_616_037,
        source_type: SourceType::Linguist,
        name: "Rascal",
        extensions: &["rsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
