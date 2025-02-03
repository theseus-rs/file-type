use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1032815495: FileFormat = FileFormat {
    id: 1_032_815_495,
    source_type: SourceType::Iana,
    name: "atsc-dynamic-event-message",
    extensions: &[],
    media_types: &["application/atsc-dynamic-event-message"],
    internal_signatures: &[],
    related_formats: &[],
};
