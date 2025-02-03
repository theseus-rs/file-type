use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4028023470: FileFormat = FileFormat {
    id: 4_028_023_470,
    source_type: SourceType::Iana,
    name: "rfc822-headers",
    extensions: &[],
    media_types: &["text/rfc822-headers"],
    internal_signatures: &[],
    related_formats: &[],
};
