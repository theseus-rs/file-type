use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_81526528: FileType = FileType {
    file_format: &FileFormat {
        id: 81_526_528,
        source_type: SourceType::Wikidata,
        name: "MicroStation Resource data",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
