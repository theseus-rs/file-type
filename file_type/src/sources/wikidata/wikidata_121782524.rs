use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121782524: FileType = FileType {
    file_format: &FileFormat {
        id: 121_782_524,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator CC Artwork 17-23",
        extensions: &["ai", "ait"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
