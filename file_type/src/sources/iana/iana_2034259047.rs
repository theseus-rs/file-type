use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2034259047: FileFormat = FileFormat {
    id: 2_034_259_047,
    source_type: SourceType::Iana,
    name: "vnd.oipf.mippvcontrolmessage+xml",
    extensions: &[],
    media_types: &["application/vnd.oipf.mippvcontrolmessage+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
