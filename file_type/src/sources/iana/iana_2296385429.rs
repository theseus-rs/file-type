use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2296385429: FileFormat = FileFormat {
    id: 2_296_385_429,
    source_type: SourceType::Iana,
    name: "vnd.irepository.package+xml",
    extensions: &[],
    media_types: &["application/vnd.irepository.package+xml"],
    signatures: &[],
    related_formats: &[],
};
