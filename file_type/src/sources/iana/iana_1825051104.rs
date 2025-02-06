use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1825051104: FileFormat = FileFormat {
    id: 1_825_051_104,
    source_type: SourceType::Iana,
    name: "vnd.bluetooth.ep.oob",
    extensions: &[],
    media_types: &["application/vnd.bluetooth.ep.oob"],
    signatures: &[],
    related_formats: &[],
};
