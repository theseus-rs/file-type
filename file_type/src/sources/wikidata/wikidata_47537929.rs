use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47537929: FileType = FileType {
    file_format: &FileFormat {
        id: 47_537_929,
        source_type: SourceType::Wikidata,
        name: "G64 GCR-encoded Disk Image Format",
        extensions: &["g41", "g64", "g71"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
