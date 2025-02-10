use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27355592: FileType = FileType {
    file_format: &FileFormat {
        id: 27_355_592,
        source_type: SourceType::Wikidata,
        name: "ADRG Geo Data File",
        extensions: &["img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
