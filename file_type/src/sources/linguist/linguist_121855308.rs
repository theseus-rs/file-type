use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_121855308: FileType = FileType {
    file_format: &FileFormat {
        id: 121_855_308,
        source_type: SourceType::Linguist,
        name: "Berry",
        extensions: &["be"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
