use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1945881547: FileFormat = FileFormat {
    id: 1_945_881_547,
    source_type: SourceType::Iana,
    name: "vnd.oma.lwm2m+tlv",
    extensions: &[],
    media_types: &["application/vnd.oma.lwm2m+tlv"],
    internal_signatures: &[],
    related_formats: &[],
};
