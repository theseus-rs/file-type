use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125809614: FileType = FileType {
    file_format: &FileFormat {
        id: 125_809_614,
        source_type: SourceType::Wikidata,
        name: "Bzip Compressed Tar Archive",
        extensions: &["tbz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
