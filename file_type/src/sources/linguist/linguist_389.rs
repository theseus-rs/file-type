use crate::format::FileFormat;

pub(crate) const LINGUIST_389: FileFormat = FileFormat {
    id: 389,
    puid: "linguist/389",
    name: "Visual Basic .NET",
    extensions: &["vb", "vbhtml"],
    media_types: &["text/x-vb"],
    internal_signatures: &[],
    related_formats: &[],
};
