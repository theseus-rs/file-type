use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27355769: FileType = FileType {
    file_format: &FileFormat {
        id: 27_355_769,
        source_type: SourceType::Wikidata,
        name: "ADRG Legend Image File",
        extensions: &["lgg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
