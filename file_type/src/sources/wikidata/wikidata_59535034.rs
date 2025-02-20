use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59535034: FileType = FileType {
    file_format: &FileFormat {
        id: 59_535_034,
        source_type: SourceType::Wikidata,
        name: "Stuffit Archive File",
        extensions: &["sit"],
        media_types: &["application/x-stuffit"],
        signatures: &[],
        related_formats: &[],
    },
};
