use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_694257130: FileFormat = FileFormat {
    id: 694_257_130,
    source_type: SourceType::Iana,
    name: "vnd.ficlab.flt",
    extensions: &[],
    media_types: &["text/vnd.ficlab.flt"],
    internal_signatures: &[],
    related_formats: &[],
};
