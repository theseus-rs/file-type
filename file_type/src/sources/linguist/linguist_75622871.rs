use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_75622871: FileFormat = FileFormat {
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
    internal_signatures: &[],
    related_formats: &[],
};
