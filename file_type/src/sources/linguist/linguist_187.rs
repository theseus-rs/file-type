use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_187: FileFormat = FileFormat {
    id: 187,
    source_type: SourceType::Linguist,
    name: "KiCad Layout",
    extensions: &["kicad_mod", "kicad_pcb", "kicad_wks"],
    media_types: &["text/x-common-lisp"],
    signatures: &[],
    related_formats: &[],
};
