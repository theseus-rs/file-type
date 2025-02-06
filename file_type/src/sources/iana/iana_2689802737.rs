use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2689802737: FileFormat = FileFormat {
    id: 2_689_802_737,
    source_type: SourceType::Iana,
    name: "rtploopback",
    extensions: &[],
    media_types: &["audio/rtploopback"],
    signatures: &[],
    related_formats: &[],
};
