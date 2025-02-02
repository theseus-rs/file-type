use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_593107205: FileFormat = FileFormat {
    id: 593_107_205,
    source_type: SourceType::Linguist,
    name: "QuickBASIC",
    extensions: &["bas"],
    media_types: &["text/x-vb"],
    internal_signatures: &[],
    related_formats: &[],
};
