use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66662128: FileType = FileType {
    file_format: &FileFormat {
        id: 66_662_128,
        source_type: SourceType::Wikidata,
        name: "Lotus Organizer",
        extensions: &["org"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
