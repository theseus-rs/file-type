use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4004639731: FileFormat = FileFormat {
    id: 4_004_639_731,
    source_type: SourceType::Iana,
    name: "vnd.bluetooth.le.oob",
    extensions: &[],
    media_types: &["application/vnd.bluetooth.le.oob"],
    internal_signatures: &[],
    related_formats: &[],
};
