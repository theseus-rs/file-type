use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66663030: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_030,
        source_type: SourceType::Wikidata,
        name: "Lotus Freelance Clip Art",
        extensions: &["sym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
