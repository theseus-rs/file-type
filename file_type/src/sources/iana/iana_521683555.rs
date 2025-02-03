use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_521683555: FileFormat = FileFormat {
    id: 521_683_555,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.provisioningtrigger",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.provisioningtrigger"],
    internal_signatures: &[],
    related_formats: &[],
};
