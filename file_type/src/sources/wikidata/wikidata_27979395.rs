use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
