use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_385: FileType = FileType {
    file_format: &FileFormat {
        id: 385,
        source_type: SourceType::Linguist,
        name: "VHDL",
        extensions: &["vhd", "vhdl", "vhf", "vhi", "vho", "vhs", "vht", "vhw"],
        media_types: &["text/x-vhdl"],
        signatures: &[],
        related_formats: &[],
    },
};
