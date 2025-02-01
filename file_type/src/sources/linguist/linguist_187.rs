use crate::format::FileFormat;

pub(crate) const LINGUIST_187: FileFormat = FileFormat {
    id: 187,
    puid: "linguist/187",
    name: "KiCad Layout",
    extensions: &["kicad_mod", "kicad_pcb", "kicad_wks"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
