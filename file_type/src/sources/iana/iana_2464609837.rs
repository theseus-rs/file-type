use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2464609837: FileFormat = FileFormat {
    id: 2_464_609_837,
    source_type: SourceType::Iana,
    name: "vnd.rip",
    extensions: &[],
    media_types: &["audio/vnd.rip"],
    signatures: &[],
    related_formats: &[],
};
