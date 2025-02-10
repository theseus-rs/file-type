use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130367574: FileType = FileType {
    file_format: &FileFormat {
        id: 130_367_574,
        source_type: SourceType::Wikidata,
        name: "Community Climate Model History Tape Format",
        extensions: &["ccm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
