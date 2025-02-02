use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_775996197: FileFormat = FileFormat {
    id: 775_996_197,
    source_type: SourceType::Linguist,
    name: "nanorc",
    extensions: &["nanorc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
