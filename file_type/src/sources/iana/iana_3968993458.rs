use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3968993458: FileFormat = FileFormat {
    id: 3_968_993_458,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.SubscriberInfo+xml",
    extensions: &[],
    media_types: &["application/EmergencyCallData.SubscriberInfo+xml"],
    signatures: &[],
    related_formats: &[],
};
