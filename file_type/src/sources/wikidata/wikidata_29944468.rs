use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29944468: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_468,
        source_type: SourceType::Wikidata,
        name: "StarOffice Impress, version 5.x",
        extensions: &["sdd"],
        media_types: &["application/vnd.stardivision.impress"],
        signatures: &[],
        related_formats: &[],
    },
};
