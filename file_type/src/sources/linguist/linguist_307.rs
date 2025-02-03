use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_307: FileFormat = FileFormat {
    id: 307,
    source_type: SourceType::Linguist,
    name: "R",
    extensions: &["r", "rd", "rsx"],
    media_types: &["text/x-rsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
