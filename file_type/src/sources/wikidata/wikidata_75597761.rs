use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_75597761: FileType = FileType {
    file_format: &FileFormat {
        id: 75_597_761,
        source_type: SourceType::Wikidata,
        name: "GeoGebra format, version 1.x",
        extensions: &["ggb"],
        media_types: &["application/vnd.geogebra.file"],
        signatures: &[],
        related_formats: &[],
    },
};
