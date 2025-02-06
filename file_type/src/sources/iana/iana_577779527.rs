use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_577779527: FileFormat = FileFormat {
    id: 577_779_527,
    source_type: SourceType::Iana,
    name: "bacnet-xdd+zip",
    extensions: &[],
    media_types: &["application/bacnet-xdd+zip"],
    signatures: &[],
    related_formats: &[],
};
