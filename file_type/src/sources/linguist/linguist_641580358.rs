use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_641580358: FileType = FileType {
    file_format: &FileFormat {
        id: 641_580_358,
        source_type: SourceType::Linguist,
        name: "Bluespec BH",
        extensions: &["bs"],
        media_types: &["text/x-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
