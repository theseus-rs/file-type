use crate::format::FileFormat;

pub(crate) const LINGUIST_22: FileFormat = FileFormat {
    id: 22,
    puid: "linguist/22",
    name: "AsciiDoc",
    extensions: &["adoc", "asc", "asciidoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
