use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_96139566: FileType = FileType {
    file_format: &FileFormat {
        id: 96_139_566,
        source_type: SourceType::Linguist,
        name: "EditorConfig",
        extensions: &["editorconfig"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
