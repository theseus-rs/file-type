use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121786280: FileType = FileType {
    file_format: &FileFormat {
        id: 121_786_280,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator CC 2020 Artwork 24.2+",
        extensions: &["ai", "ait"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
