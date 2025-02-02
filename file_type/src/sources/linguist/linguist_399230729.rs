use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_399230729: FileFormat = FileFormat {
    id: 399_230_729,
    source_type: SourceType::Linguist,
    name: "VBA",
    extensions: &["bas", "cls", "frm", "vba"],
    media_types: &["text/x-vb"],
    internal_signatures: &[],
    related_formats: &[],
};
