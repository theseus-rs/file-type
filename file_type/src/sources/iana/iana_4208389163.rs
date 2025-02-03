use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4208389163: FileFormat = FileFormat {
    id: 4_208_389_163,
    source_type: SourceType::Iana,
    name: "jscalendar+json",
    extensions: &[],
    media_types: &["application/jscalendar+json"],
    internal_signatures: &[],
    related_formats: &[],
};
