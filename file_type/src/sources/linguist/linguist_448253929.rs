use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_448253929: FileFormat = FileFormat {
    id: 448_253_929,
    source_type: SourceType::Linguist,
    name: "MLIR",
    extensions: &["mlir"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
