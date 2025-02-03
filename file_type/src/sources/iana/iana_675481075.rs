use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_675481075: FileFormat = FileFormat {
    id: 675_481_075,
    source_type: SourceType::Iana,
    name: "vnd.collection.doc+json",
    extensions: &[],
    media_types: &["application/vnd.collection.doc+json"],
    internal_signatures: &[],
    related_formats: &[],
};
