use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_95110458: FileType = FileType {
    file_format: &FileFormat {
        id: 95_110_458,
        source_type: SourceType::Linguist,
        name: "Glimmer TS",
        extensions: &["gts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
