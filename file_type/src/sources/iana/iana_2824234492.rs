use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2824234492: FileFormat = FileFormat {
    id: 2_824_234_492,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
