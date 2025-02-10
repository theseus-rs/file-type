use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125915605: FileType = FileType {
    file_format: &FileFormat {
        id: 125_915_605,
        source_type: SourceType::Wikidata,
        name: "Melco OFM Project pre v.11",
        extensions: &["ofm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
