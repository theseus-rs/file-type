use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966952: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_952,
        source_type: SourceType::Wikidata,
        name: "SSF",
        extensions: &["minissf", "ssf", "ssflib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
