use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_368: FileFormat = FileFormat {
    id: 368,
    source_type: SourceType::Linguist,
    name: "Tcsh",
    extensions: &["csh", "tcsh"],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
