use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27355565: FileType = FileType {
    file_format: &FileFormat {
        id: 27_355_565,
        source_type: SourceType::Wikidata,
        name: "ADRG Quality File",
        extensions: &["qal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
