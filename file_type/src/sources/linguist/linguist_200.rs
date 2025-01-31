use crate::format::FileFormat;

pub(crate) const LINGUIST_200: FileFormat = FileFormat {
    id: 200,
    puid: "linguist/200",
    name: "LilyPond",
    extensions: &["ily", "ly"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
