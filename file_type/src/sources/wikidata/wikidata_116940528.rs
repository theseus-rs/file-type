use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116940528: FileType = FileType {
    file_format: &FileFormat {
        id: 116_940_528,
        source_type: SourceType::Wikidata,
        name: "AccessData Recovery Session",
        extensions: &["adr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
