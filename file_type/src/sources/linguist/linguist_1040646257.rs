use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_1040646257: FileType = FileType {
    file_format: &FileFormat {
        id: 1_040_646_257,
        source_type: SourceType::Linguist,
        name: "LigoLANG",
        extensions: &["ligo"],
        media_types: &["text/x-pascal"],
        signatures: &[],
        related_formats: &[],
    },
};
