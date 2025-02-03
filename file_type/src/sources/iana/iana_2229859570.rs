use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2229859570: FileFormat = FileFormat {
    id: 2_229_859_570,
    source_type: SourceType::Iana,
    name: "sensml+xml",
    extensions: &[],
    media_types: &["application/sensml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
