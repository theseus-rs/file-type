use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_130: FileType = FileType {
    file_format: &FileFormat {
        id: 130,
        source_type: SourceType::Linguist,
        name: "Glyph",
        extensions: &["glf"],
        media_types: &["text/x-tcl"],
        signatures: &[],
        related_formats: &[],
    },
};
