use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1011219020: FileFormat = FileFormat {
    id: 1_011_219_020,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-dialog+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-dialog+xml"],
    signatures: &[],
    related_formats: &[],
};
