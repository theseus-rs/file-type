use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_14081048: FileFormat = FileFormat {
    id: 14_081_048,
    source_type: SourceType::Iana,
    name: "vnd.parasolid.transmit.text",
    extensions: &[],
    media_types: &["model/vnd.parasolid.transmit.text"],
    signatures: &[],
    related_formats: &[],
};
