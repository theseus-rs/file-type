use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126960642: FileType = FileType {
    file_format: &FileFormat {
        id: 126_960_642,
        source_type: SourceType::Wikidata,
        name: "SystemVerilog Source Code File",
        extensions: &["sv"],
        media_types: &["text/x-systemverilog"],
        signatures: &[],
        related_formats: &[],
    },
};
