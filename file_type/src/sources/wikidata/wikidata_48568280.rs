use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48568280: FileType = FileType {
    file_format: &FileFormat {
        id: 48_568_280,
        source_type: SourceType::Wikidata,
        name: "Lightwright 5 Show File",
        extensions: &["lw5"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
