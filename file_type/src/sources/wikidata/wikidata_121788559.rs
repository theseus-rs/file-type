use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121788559: FileType = FileType {
    file_format: &FileFormat {
        id: 121_788_559,
        source_type: SourceType::Wikidata,
        name: "Leapfrog Geo 3D Scene Format",
        extensions: &["lfsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
