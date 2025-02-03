use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_361957890: FileFormat = FileFormat {
    id: 361_957_890,
    source_type: SourceType::Iana,
    name: "vnd.syncml.dm+wbxml",
    extensions: &[],
    media_types: &["application/vnd.syncml.dm+wbxml"],
    internal_signatures: &[],
    related_formats: &[],
};
