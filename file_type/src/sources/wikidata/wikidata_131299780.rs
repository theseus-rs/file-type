use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131299780: FileType = FileType {
    file_format: &FileFormat {
        id: 131_299_780,
        source_type: SourceType::Wikidata,
        name: "ThingsDB file format",
        extensions: &["ti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
