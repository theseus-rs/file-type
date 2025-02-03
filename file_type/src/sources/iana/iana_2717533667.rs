use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2717533667: FileFormat = FileFormat {
    id: 2_717_533_667,
    source_type: SourceType::Iana,
    name: "example",
    extensions: &[],
    media_types: &["text/example"],
    internal_signatures: &[],
    related_formats: &[],
};
