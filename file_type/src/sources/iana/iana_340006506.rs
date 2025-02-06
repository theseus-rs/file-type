use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_340006506: FileFormat = FileFormat {
    id: 340_006_506,
    source_type: SourceType::Iana,
    name: "H264-SVC",
    extensions: &[],
    media_types: &["video/H264-SVC"],
    signatures: &[],
    related_formats: &[],
};
