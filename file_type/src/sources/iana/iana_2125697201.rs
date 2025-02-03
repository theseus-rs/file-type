use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2125697201: FileFormat = FileFormat {
    id: 2_125_697_201,
    source_type: SourceType::Iana,
    name: "vnd.3M.Post-it-Notes",
    extensions: &[],
    media_types: &["application/vnd.3M.Post-it-Notes"],
    internal_signatures: &[],
    related_formats: &[],
};
