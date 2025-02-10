use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66310997: FileType = FileType {
    file_format: &FileFormat {
        id: 66_310_997,
        source_type: SourceType::Wikidata,
        name: "Favorite Files",
        extensions: &["mfv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
