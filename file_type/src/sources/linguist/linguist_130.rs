use crate::format::FileFormat;

pub(crate) const LINGUIST_130: FileFormat = FileFormat {
    id: 130,
    puid: "linguist/130",
    name: "Glyph",
    extensions: &["glf"],
    media_types: &["text/x-tcl"],
    internal_signatures: &[],
    related_formats: &[],
};
