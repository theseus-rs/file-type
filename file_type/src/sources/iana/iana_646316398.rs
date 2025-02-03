use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_646316398: FileFormat = FileFormat {
    id: 646_316_398,
    source_type: SourceType::Iana,
    name: "vnd.ffsns",
    extensions: &[],
    media_types: &["application/vnd.ffsns"],
    internal_signatures: &[],
    related_formats: &[],
};
