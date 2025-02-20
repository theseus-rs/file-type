use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_75710135: FileType = FileType {
    file_format: &FileFormat {
        id: 75_710_135,
        source_type: SourceType::Wikidata,
        name: "GeoGebra format, version 4",
        extensions: &["ggb"],
        media_types: &["application/vnd.geogebra.file"],
        signatures: &[],
        related_formats: &[],
    },
};
