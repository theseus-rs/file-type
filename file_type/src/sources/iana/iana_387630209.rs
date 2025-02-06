use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_387630209: FileFormat = FileFormat {
    id: 387_630_209,
    source_type: SourceType::Iana,
    name: "vcard+xml",
    extensions: &[],
    media_types: &["application/vcard+xml"],
    signatures: &[],
    related_formats: &[],
};
