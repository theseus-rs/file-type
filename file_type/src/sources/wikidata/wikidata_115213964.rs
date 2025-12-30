use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115213964: FileType = FileType {
    file_format: &FileFormat {
        id: 115_213_964,
        source_type: SourceType::Wikidata,
        name: "EditorConfig",
        extensions: &["editorconfig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
