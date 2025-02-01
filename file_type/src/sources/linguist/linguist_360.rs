use crate::format::FileFormat;

pub(crate) const LINGUIST_360: FileFormat = FileFormat {
    id: 360,
    puid: "linguist/360",
    name: "SubRip Text",
    extensions: &["srt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
