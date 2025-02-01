use crate::format::FileFormat;

pub(crate) const LINGUIST_75622871: FileFormat = FileFormat {
    id: 75_622_871,
    puid: "linguist/75622871",
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
