use crate::format::FileFormat;

pub(crate) const LINGUIST_275: FileFormat = FileFormat {
    id: 275,
    puid: "linguist/275",
    name: "POV-Ray SDL",
    extensions: &["inc", "pov"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
