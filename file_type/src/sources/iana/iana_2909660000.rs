use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2909660000: FileFormat = FileFormat {
    id: 2_909_660_000,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
