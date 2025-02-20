use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
