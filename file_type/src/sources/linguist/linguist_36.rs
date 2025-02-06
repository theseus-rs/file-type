use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_36: FileFormat = FileFormat {
    id: 36,
    source_type: SourceType::Linguist,
    name: "Bluespec",
    extensions: &["bsv"],
    media_types: &["text/x-systemverilog"],
    signatures: &[],
    related_formats: &[],
};
