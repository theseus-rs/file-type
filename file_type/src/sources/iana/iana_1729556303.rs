use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1729556303: FileFormat = FileFormat {
    id: 1_729_556_303,
    source_type: SourceType::Iana,
    name: "GSM-HR-08",
    extensions: &[],
    media_types: &["audio/GSM-HR-08"],
    internal_signatures: &[],
    related_formats: &[],
};
