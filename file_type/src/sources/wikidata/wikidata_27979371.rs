use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979371: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_371,
        source_type: SourceType::Wikidata,
        name: "EBU Timed Text",
        extensions: &["ttml"],
        media_types: &["application/ttml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
