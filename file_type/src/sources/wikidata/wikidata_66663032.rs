use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66663032: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_032,
        source_type: SourceType::Wikidata,
        name: "Lotus Freelance Diagram",
        extensions: &["dgm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
