use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_242: FileFormat = FileFormat {
    id: 242,
    source_type: SourceType::Linguist,
    name: "NSIS",
    extensions: &["nsh", "nsi"],
    media_types: &["text/x-nsis"],
    internal_signatures: &[],
    related_formats: &[],
};
