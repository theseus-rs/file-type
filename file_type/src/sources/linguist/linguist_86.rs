use crate::format::FileFormat;

pub(crate) const LINGUIST_86: FileFormat = FileFormat {
    id: 86,
    puid: "linguist/86",
    name: "Darcs Patch",
    extensions: &["darcspatch", "dpatch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
