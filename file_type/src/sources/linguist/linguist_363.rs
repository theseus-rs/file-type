use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_363: FileFormat = FileFormat {
    id: 363,
    source_type: SourceType::Linguist,
    name: "SystemVerilog",
    extensions: &["sv", "svh", "vh"],
    media_types: &["text/x-systemverilog"],
    internal_signatures: &[],
    related_formats: &[],
};
