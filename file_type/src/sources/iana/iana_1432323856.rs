use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1432323856: FileFormat = FileFormat {
    id: 1_432_323_856,
    source_type: SourceType::Iana,
    name: "vnd.paos.xml",
    extensions: &[],
    media_types: &["application/vnd.paos.xml"],
    internal_signatures: &[],
    related_formats: &[],
};
