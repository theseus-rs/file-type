use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113556932: FileType = FileType {
    file_format: &FileFormat {
        id: 113_556_932,
        source_type: SourceType::Wikidata,
        name: "Duplicator Image format",
        extensions: &["dao"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
