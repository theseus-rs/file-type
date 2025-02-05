use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1704586580: FileFormat = FileFormat {
    id: 1_704_586_580,
    source_type: SourceType::Iana,
    name: "vnd.cendio.thinlinc.clientconf",
    extensions: &[],
    media_types: &["application/vnd.cendio.thinlinc.clientconf"],
    signatures: &[],
    related_formats: &[],
};
