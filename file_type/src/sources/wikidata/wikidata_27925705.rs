use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27925705: FileType = FileType {
    file_format: &FileFormat {
        id: 27_925_705,
        source_type: SourceType::Wikidata,
        name: "DTED Readme file",
        extensions: &["me"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
