use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_8041961: FileType = FileType {
    file_format: &FileFormat {
        id: 8_041_961,
        source_type: SourceType::Wikidata,
        name: "eXtensible Graph Markup and Modeling Language",
        extensions: &["XGMML"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
