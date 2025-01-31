use crate::format::FileFormat;

pub(crate) const LINGUIST_76: FileFormat = FileFormat {
    id: 76,
    puid: "linguist/76",
    name: "Gherkin",
    extensions: &["feature", "story"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
