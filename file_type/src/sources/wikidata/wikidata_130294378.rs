use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130294378: FileType = FileType {
    file_format: &FileFormat {
        id: 130_294_378,
        source_type: SourceType::Wikidata,
        name: "MIPS file format",
        extensions: &["mips"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
