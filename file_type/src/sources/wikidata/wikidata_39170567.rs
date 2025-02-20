use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_39170567: FileType = FileType {
    file_format: &FileFormat {
        id: 39_170_567,
        source_type: SourceType::Wikidata,
        name: "GeoGebra tool",
        extensions: &["ggt"],
        media_types: &["application/vnd.geogebra.tool"],
        signatures: &[],
        related_formats: &[],
    },
};
