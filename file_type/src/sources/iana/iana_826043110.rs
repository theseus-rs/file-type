use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_826043110: FileFormat = FileFormat {
    id: 826_043_110,
    source_type: SourceType::Iana,
    name: "LPC",
    extensions: &[],
    media_types: &["audio/LPC"],
    internal_signatures: &[],
    related_formats: &[],
};
