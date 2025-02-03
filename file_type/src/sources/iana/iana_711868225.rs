use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_711868225: FileFormat = FileFormat {
    id: 711_868_225,
    source_type: SourceType::Iana,
    name: "H264-RCDO",
    extensions: &[],
    media_types: &["video/H264-RCDO"],
    internal_signatures: &[],
    related_formats: &[],
};
