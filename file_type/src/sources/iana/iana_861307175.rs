use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_861307175: FileFormat = FileFormat {
    id: 861_307_175,
    source_type: SourceType::Iana,
    name: "alto-networkmapfilter+json",
    extensions: &[],
    media_types: &["application/alto-networkmapfilter+json"],
    signatures: &[],
    related_formats: &[],
};
