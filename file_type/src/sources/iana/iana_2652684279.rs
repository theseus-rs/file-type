use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2652684279: FileFormat = FileFormat {
    id: 2_652_684_279,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.eCall.MSD",
    extensions: &[],
    media_types: &["application/EmergencyCallData.eCall.MSD"],
    signatures: &[],
    related_formats: &[],
};
