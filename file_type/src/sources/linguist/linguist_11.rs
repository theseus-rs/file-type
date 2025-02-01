use crate::format::FileFormat;

pub(crate) const LINGUIST_11: FileFormat = FileFormat {
    id: 11,
    puid: "linguist/11",
    name: "Ada",
    extensions: &["ada", "adb", "ads"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
