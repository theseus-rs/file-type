use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3745771565: FileFormat = FileFormat {
    id: 3_745_771_565,
    source_type: SourceType::Iana,
    name: "BMPEG",
    extensions: &[],
    media_types: &["video/BMPEG"],
    internal_signatures: &[],
    related_formats: &[],
};
