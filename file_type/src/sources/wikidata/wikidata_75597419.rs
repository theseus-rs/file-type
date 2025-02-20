use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_75597419: FileType = FileType {
    file_format: &FileFormat {
        id: 75_597_419,
        source_type: SourceType::Wikidata,
        name: "GeoGebra format, version 2.0",
        extensions: &["ggb"],
        media_types: &["application/vnd.geogebra.file"],
        signatures: &[],
        related_formats: &[],
    },
};
