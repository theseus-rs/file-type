use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_347: FileFormat = FileFormat {
    id: 347,
    source_type: SourceType::Linguist,
    name: "ShellSession",
    extensions: &["sh-session"],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
