use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_153739399: FileFormat = FileFormat {
    id: 153_739_399,
    source_type: SourceType::Linguist,
    name: "OpenQASM",
    extensions: &["qasm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
