use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_388: FileFormat = FileFormat {
    id: 388,
    source_type: SourceType::Linguist,
    name: "Vim Script",
    extensions: &["vba", "vim", "vimrc", "vmb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
