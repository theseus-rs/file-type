use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112668672: FileType = FileType {
    file_format: &FileFormat {
        id: 112_668_672,
        source_type: SourceType::Wikidata,
        name: "Lightscape Blocks",
        extensions: &["blk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
