use crate::format::FileFormat;

pub(crate) const LINGUIST_353: FileFormat = FileFormat {
    id: 353,
    puid: "linguist/353",
    name: "Smarty",
    extensions: &["tpl"],
    media_types: &["text/x-smarty"],
    internal_signatures: &[],
    related_formats: &[],
};
