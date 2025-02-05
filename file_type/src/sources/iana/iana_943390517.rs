use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_943390517: FileFormat = FileFormat {
    id: 943_390_517,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.ltkm",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.ltkm"],
    signatures: &[],
    related_formats: &[],
};
