use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2859263887: FileFormat = FileFormat {
    id: 2_859_263_887,
    source_type: SourceType::Iana,
    name: "vnd.lotus-approach",
    extensions: &[],
    media_types: &["application/vnd.lotus-approach"],
    internal_signatures: &[],
    related_formats: &[],
};
