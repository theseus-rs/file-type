use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_353: FileFormat = FileFormat {
    id: 353,
    source_type: SourceType::Linguist,
    name: "Smarty",
    extensions: &["tpl"],
    media_types: &["text/x-smarty"],
    signatures: &[],
    related_formats: &[],
};
