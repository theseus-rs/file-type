use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_687511714: FileFormat = FileFormat {
    id: 687_511_714,
    source_type: SourceType::Linguist,
    name: "ShellCheck Config",
    extensions: &[],
    media_types: &["text/x-properties"],
    internal_signatures: &[],
    related_formats: &[],
};
