use crate::format::FileFormat;

pub(crate) const LINGUIST_117: FileFormat = FileFormat {
    id: 117,
    puid: "linguist/117",
    name: "G-code",
    extensions: &["cnc", "g", "gco", "gcode"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
