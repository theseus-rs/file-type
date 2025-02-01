use crate::format::FileFormat;

pub(crate) const LINGUIST_232: FileFormat = FileFormat {
    id: 232,
    puid: "linguist/232",
    name: "Mirah",
    extensions: &["druby", "duby", "mirah"],
    media_types: &["text/x-ruby"],
    internal_signatures: &[],
    related_formats: &[],
};
