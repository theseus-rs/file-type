use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_472308069: FileType = FileType {
    file_format: &FileFormat {
        id: 472_308_069,
        source_type: SourceType::Linguist,
        name: "QuakeC",
        extensions: &["qc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
