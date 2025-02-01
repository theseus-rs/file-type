use crate::format::FileFormat;

pub(crate) const LINGUIST_140: FileFormat = FileFormat {
    id: 140,
    puid: "linguist/140",
    name: "Graphviz (DOT)",
    extensions: &["dot", "gv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
