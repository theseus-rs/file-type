use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
