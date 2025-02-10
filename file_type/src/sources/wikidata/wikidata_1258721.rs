use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1258721: FileType = FileType {
    file_format: &FileFormat {
        id: 1_258_721,
        source_type: SourceType::Wikidata,
        name: "NFO",
        extensions: &["nfo"],
        media_types: &["text/x-nfo"],
        signatures: &[],
        related_formats: &[],
    },
};
