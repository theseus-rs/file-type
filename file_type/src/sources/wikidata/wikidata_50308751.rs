use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50308751: FileType = FileType {
    file_format: &FileFormat {
        id: 50_308_751,
        source_type: SourceType::Wikidata,
        name: "MIME Email format",
        extensions: &["eml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
