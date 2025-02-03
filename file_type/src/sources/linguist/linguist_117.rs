use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_117: FileFormat = FileFormat {
    id: 117,
    source_type: SourceType::Linguist,
    name: "G-code",
    extensions: &["cnc", "g", "gco", "gcode"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
