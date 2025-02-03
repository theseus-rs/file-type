use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_322: FileFormat = FileFormat {
    id: 322,
    source_type: SourceType::Linguist,
    name: "Ren'Py",
    extensions: &["rpy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
