use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3510790454: FileFormat = FileFormat {
    id: 3_510_790_454,
    source_type: SourceType::Iana,
    name: "vnd.vidsoft.vidconference",
    extensions: &[],
    media_types: &["application/vnd.vidsoft.vidconference"],
    signatures: &[],
    related_formats: &[],
};
