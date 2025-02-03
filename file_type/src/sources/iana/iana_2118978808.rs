use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2118978808: FileFormat = FileFormat {
    id: 2_118_978_808,
    source_type: SourceType::Iana,
    name: "vnd.syncml.dm+xml",
    extensions: &[],
    media_types: &["application/vnd.syncml.dm+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
