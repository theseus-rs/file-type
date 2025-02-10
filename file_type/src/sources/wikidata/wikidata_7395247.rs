use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7395247: FileType = FileType {
    file_format: &FileFormat {
        id: 7_395_247,
        source_type: SourceType::Wikidata,
        name: "SYBYL line notation",
        extensions: &["sln"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
