use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979395: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_395,
        source_type: SourceType::Wikidata,
        name: "GIFV",
        extensions: &["gifv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
