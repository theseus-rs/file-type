use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_73515052: FileType = FileType {
    file_format: &FileFormat {
        id: 73_515_052,
        source_type: SourceType::Wikidata,
        name: "Microsoft Printer Definition",
        extensions: &["prd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
