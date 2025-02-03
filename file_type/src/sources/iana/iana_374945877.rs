use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_374945877: FileFormat = FileFormat {
    id: 374_945_877,
    source_type: SourceType::Iana,
    name: "xcap-el+xml",
    extensions: &[],
    media_types: &["application/xcap-el+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
