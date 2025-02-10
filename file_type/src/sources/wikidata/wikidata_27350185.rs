use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27350185: FileType = FileType {
    file_format: &FileFormat {
        id: 27_350_185,
        source_type: SourceType::Wikidata,
        name: "ADRG Test Patch Image File",
        extensions: &["cph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
