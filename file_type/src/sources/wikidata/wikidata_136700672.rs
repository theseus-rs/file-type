use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136700672: FileType = FileType {
    file_format: &FileFormat {
        id: 136_700_672,
        source_type: SourceType::Wikidata,
        name: "Nintendo Wii Optical Disc image",
        extensions: &["iso"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
