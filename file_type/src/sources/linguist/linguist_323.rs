use crate::format::FileFormat;

pub(crate) const LINGUIST_323: FileFormat = FileFormat {
    id: 323,
    puid: "linguist/323",
    name: "RenderScript",
    extensions: &["rs", "rsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
