use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_363: FileType = FileType {
    file_format: &FileFormat {
        id: 363,
        source_type: SourceType::Linguist,
        name: "SystemVerilog",
        extensions: &["sv", "svh", "vh"],
        media_types: &["text/x-systemverilog"],
        signatures: &[],
        related_formats: &[],
    },
};
