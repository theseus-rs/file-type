use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_387: FileType = FileType {
    file_format: &FileFormat {
        id: 387,
        source_type: SourceType::Linguist,
        name: "Verilog",
        extensions: &["v", "veo"],
        media_types: &["text/x-verilog"],
        signatures: &[],
        related_formats: &[],
    },
};
