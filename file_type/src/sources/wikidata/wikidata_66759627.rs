use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66759627: FileType = FileType {
    file_format: &FileFormat {
        id: 66_759_627,
        source_type: SourceType::Wikidata,
        name: "Space-delimited formatted text file",
        extensions: &["prn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
