use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127378243: FileType = FileType {
    file_format: &FileFormat {
        id: 127_378_243,
        source_type: SourceType::Wikidata,
        name: "FreeBASIC Header File",
        extensions: &["bi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
