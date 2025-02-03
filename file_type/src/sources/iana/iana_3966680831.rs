use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3966680831: FileFormat = FileFormat {
    id: 3_966_680_831,
    source_type: SourceType::Iana,
    name: "ulpfec",
    extensions: &[],
    media_types: &["video/ulpfec"],
    internal_signatures: &[],
    related_formats: &[],
};
