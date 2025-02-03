use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2325811780: FileFormat = FileFormat {
    id: 2_325_811_780,
    source_type: SourceType::Iana,
    name: "senml-etch+json",
    extensions: &[],
    media_types: &["application/senml-etch+json"],
    internal_signatures: &[],
    related_formats: &[],
};
