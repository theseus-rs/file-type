use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_310828396: FileType = FileType {
    file_format: &FileFormat {
        id: 310_828_396,
        source_type: SourceType::Linguist,
        name: "Gemini",
        extensions: &["gmi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
