use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4078358736: FileFormat = FileFormat {
    id: 4_078_358_736,
    source_type: SourceType::Iana,
    name: "vnd.ms-powerpoint",
    extensions: &[],
    media_types: &["application/vnd.ms-powerpoint"],
    signatures: &[],
    related_formats: &[],
};
