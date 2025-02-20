use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_75598901: FileType = FileType {
    file_format: &FileFormat {
        id: 75_598_901,
        source_type: SourceType::Wikidata,
        name: "GeoGebra format, version 3",
        extensions: &["ggb"],
        media_types: &["application/vnd.geogebra.file"],
        signatures: &[],
        related_formats: &[],
    },
};
