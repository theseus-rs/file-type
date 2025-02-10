use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117156378: FileType = FileType {
    file_format: &FileFormat {
        id: 117_156_378,
        source_type: SourceType::Wikidata,
        name: "Pyro audio CD project",
        extensions: &["cwa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
