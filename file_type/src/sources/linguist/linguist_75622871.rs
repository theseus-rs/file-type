use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_75622871: FileType = FileType {
    file_format: &FileFormat {
        id: 75_622_871,
        source_type: SourceType::Linguist,
        name: "XML Property List",
        extensions: &[
            "plist",
            "stTheme",
            "tmCommand",
            "tmLanguage",
            "tmPreferences",
            "tmSnippet",
            "tmTheme",
        ],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
