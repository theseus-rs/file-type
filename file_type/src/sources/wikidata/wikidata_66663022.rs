use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66663022: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_022,
        source_type: SourceType::Wikidata,
        name: "Lotus Freelance 2.x for OS/2",
        extensions: &["prs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
