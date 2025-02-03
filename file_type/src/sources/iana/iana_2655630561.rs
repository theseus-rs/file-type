use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2655630561: FileFormat = FileFormat {
    id: 2_655_630_561,
    source_type: SourceType::Iana,
    name: "vnd.syncml.dmtnds+wbxml",
    extensions: &[],
    media_types: &["application/vnd.syncml.dmtnds+wbxml"],
    internal_signatures: &[],
    related_formats: &[],
};
