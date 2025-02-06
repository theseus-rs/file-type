use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_86: FileFormat = FileFormat {
    id: 86,
    source_type: SourceType::Linguist,
    name: "Darcs Patch",
    extensions: &["darcspatch", "dpatch"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
