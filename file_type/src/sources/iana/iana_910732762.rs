use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_910732762: FileFormat = FileFormat {
    id: 910_732_762,
    source_type: SourceType::Iana,
    name: "vnd.kidspiration",
    extensions: &[],
    media_types: &["application/vnd.kidspiration"],
    internal_signatures: &[],
    related_formats: &[],
};
