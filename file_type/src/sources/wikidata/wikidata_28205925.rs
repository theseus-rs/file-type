use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205925: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_925,
        source_type: SourceType::Wikidata,
        name: "Doodle",
        extensions: &["doo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
