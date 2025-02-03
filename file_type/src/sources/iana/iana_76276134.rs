use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_76276134: FileFormat = FileFormat {
    id: 76_276_134,
    source_type: SourceType::Iana,
    name: "timestamp-reply",
    extensions: &[],
    media_types: &["application/timestamp-reply"],
    internal_signatures: &[],
    related_formats: &[],
};
