use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1005760972: FileFormat = FileFormat {
    id: 1_005_760_972,
    source_type: SourceType::Iana,
    name: "vnd.epson.msf",
    extensions: &[],
    media_types: &["application/vnd.epson.msf"],
    internal_signatures: &[],
    related_formats: &[],
};
