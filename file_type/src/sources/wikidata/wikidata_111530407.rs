use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111530407: FileType = FileType {
    file_format: &FileFormat {
        id: 111_530_407,
        source_type: SourceType::Wikidata,
        name: "Esri ArcExplorer project file",
        extensions: &["aep"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
