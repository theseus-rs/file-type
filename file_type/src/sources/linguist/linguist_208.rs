use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_208: FileFormat = FileFormat {
    id: 208,
    source_type: SourceType::Linguist,
    name: "LiveScript",
    extensions: &["_ls", "ls"],
    media_types: &["text/x-livescript"],
    internal_signatures: &[],
    related_formats: &[],
};
