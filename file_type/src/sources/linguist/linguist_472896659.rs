use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_472896659: FileFormat = FileFormat {
    id: 472_896_659,
    source_type: SourceType::Linguist,
    name: "FreeBASIC",
    extensions: &["bas", "bi"],
    media_types: &["text/x-vb"],
    internal_signatures: &[],
    related_formats: &[],
};
