use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3163242823: FileFormat = FileFormat {
    id: 3_163_242_823,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
    ],
    signatures: &[],
    related_formats: &[],
};
