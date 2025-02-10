use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4208389163: FileType = FileType {
    file_format: &FileFormat {
        id: 4_208_389_163,
        source_type: SourceType::Iana,
        name: "jscalendar+json",
        extensions: &[],
        media_types: &["application/jscalendar+json"],
        signatures: &[],
        related_formats: &[],
    },
};
