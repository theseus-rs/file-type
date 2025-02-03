use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_440186036: FileFormat = FileFormat {
    id: 440_186_036,
    source_type: SourceType::Iana,
    name: "vnd.wap.wbmp",
    extensions: &[],
    media_types: &["image/vnd.wap.wbmp"],
    internal_signatures: &[],
    related_formats: &[],
};
