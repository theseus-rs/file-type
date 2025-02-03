use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_130: FileFormat = FileFormat {
    id: 130,
    source_type: SourceType::Linguist,
    name: "Glyph",
    extensions: &["glf"],
    media_types: &["text/x-tcl"],
    internal_signatures: &[],
    related_formats: &[],
};
