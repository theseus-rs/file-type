use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_275: FileType = FileType {
    file_format: &FileFormat {
        id: 275,
        source_type: SourceType::Linguist,
        name: "POV-Ray SDL",
        extensions: &["inc", "pov"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
