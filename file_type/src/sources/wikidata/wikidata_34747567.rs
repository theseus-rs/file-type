use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34747567: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_567,
        source_type: SourceType::Wikidata,
        name: "Stronghold GM1",
        extensions: &["gm1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
