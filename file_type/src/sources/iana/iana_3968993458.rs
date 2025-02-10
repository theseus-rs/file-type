use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3968993458: FileType = FileType {
    file_format: &FileFormat {
        id: 3_968_993_458,
        source_type: SourceType::Iana,
        name: "EmergencyCallData.SubscriberInfo+xml",
        extensions: &[],
        media_types: &["application/EmergencyCallData.SubscriberInfo+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
