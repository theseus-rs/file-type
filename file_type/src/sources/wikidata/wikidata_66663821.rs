use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66663821: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_821,
        source_type: SourceType::Wikidata,
        name: "Hewlett-Packard Graphics Language format",
        extensions: &["hgl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
