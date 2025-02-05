use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_430666742: FileFormat = FileFormat {
    id: 430_666_742,
    source_type: SourceType::Iana,
    name: "vnd.etsi.simservs+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.simservs+xml"],
    signatures: &[],
    related_formats: &[],
};
