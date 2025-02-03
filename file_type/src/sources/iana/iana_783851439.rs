use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_783851439: FileFormat = FileFormat {
    id: 783_851_439,
    source_type: SourceType::Iana,
    name: "uri-list",
    extensions: &[],
    media_types: &["text/uri-list"],
    internal_signatures: &[],
    related_formats: &[],
};
