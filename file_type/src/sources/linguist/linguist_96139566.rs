use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_96139566: FileFormat = FileFormat {
    id: 96_139_566,
    source_type: SourceType::Linguist,
    name: "EditorConfig",
    extensions: &["editorconfig"],
    media_types: &["text/x-properties"],
    internal_signatures: &[],
    related_formats: &[],
};
