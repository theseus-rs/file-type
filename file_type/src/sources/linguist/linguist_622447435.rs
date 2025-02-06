use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_622447435: FileFormat = FileFormat {
    id: 622_447_435,
    source_type: SourceType::Linguist,
    name: "KiCad Schematic",
    extensions: &["kicad_sch", "sch"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
