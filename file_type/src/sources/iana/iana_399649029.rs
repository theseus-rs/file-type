use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_399649029: FileFormat = FileFormat {
    id: 399_649_029,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.stkm",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.stkm"],
    internal_signatures: &[],
    related_formats: &[],
};
