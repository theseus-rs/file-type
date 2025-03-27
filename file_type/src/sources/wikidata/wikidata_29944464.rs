use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29944464: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_464,
        source_type: SourceType::Wikidata,
        name: "StarOffice Impress, version 4.x",
        extensions: &["sdd"],
        media_types: &["application/vnd.stardivision.impress"],
        signatures: &[],
        related_formats: &[],
    },
};
