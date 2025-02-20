use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111511710: FileType = FileType {
    file_format: &FileFormat {
        id: 111_511_710,
        source_type: SourceType::Wikidata,
        name: "TGIF File Format",
        extensions: &["obj", "tgif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
