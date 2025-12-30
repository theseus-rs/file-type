use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136400228: FileType = FileType {
    file_format: &FileFormat {
        id: 136_400_228,
        source_type: SourceType::Wikidata,
        name: "CityGML file format",
        extensions: &["gml", "xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
