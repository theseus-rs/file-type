use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3749448474: FileFormat = FileFormat {
    id: 3_749_448_474,
    source_type: SourceType::Iana,
    name: "vnd.ms-windows.nwprinting.oob",
    extensions: &[],
    media_types: &["application/vnd.ms-windows.nwprinting.oob"],
    internal_signatures: &[],
    related_formats: &[],
};
