use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50376352: FileType = FileType {
    file_format: &FileFormat {
        id: 50_376_352,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcGlobe Document",
        extensions: &["3dd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
