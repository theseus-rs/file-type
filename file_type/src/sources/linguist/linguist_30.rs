use crate::format::FileFormat;

pub(crate) const LINGUIST_30: FileFormat = FileFormat {
    id: 30,
    puid: "linguist/30",
    name: "Befunge",
    extensions: &["befunge", "bf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
