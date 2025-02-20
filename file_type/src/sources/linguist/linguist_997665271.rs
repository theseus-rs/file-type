use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_997665271: FileType = FileType {
    file_format: &FileFormat {
        id: 997_665_271,
        source_type: SourceType::Linguist,
        name: "Glyph Bitmap Distribution Format",
        extensions: &["bdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
