use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979367: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_367,
        source_type: SourceType::Wikidata,
        name: "ReScene",
        extensions: &["srr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
