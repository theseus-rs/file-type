use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_173616037: FileFormat = FileFormat {
    id: 173_616_037,
    source_type: SourceType::Linguist,
    name: "Rascal",
    extensions: &["rsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
