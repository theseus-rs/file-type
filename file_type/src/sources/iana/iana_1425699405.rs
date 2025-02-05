use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1425699405: FileFormat = FileFormat {
    id: 1_425_699_405,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-dialog-speech+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-dialog-speech+xml"],
    signatures: &[],
    related_formats: &[],
};
