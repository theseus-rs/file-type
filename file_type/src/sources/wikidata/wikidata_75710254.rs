use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_75710254: FileType = FileType {
    file_format: &FileFormat {
        id: 75_710_254,
        source_type: SourceType::Wikidata,
        name: "GeoGebra format, version 5",
        extensions: &["ggb"],
        media_types: &["application/vnd.geogebra.file"],
        signatures: &[],
        related_formats: &[],
    },
};
