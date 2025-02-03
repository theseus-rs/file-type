use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2996835525: FileFormat = FileFormat {
    id: 2_996_835_525,
    source_type: SourceType::Iana,
    name: "vnd.eu.kasparian.car+json",
    extensions: &[],
    media_types: &["application/vnd.eu.kasparian.car+json"],
    internal_signatures: &[],
    related_formats: &[],
};
