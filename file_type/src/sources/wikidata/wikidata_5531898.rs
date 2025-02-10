use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5531898: FileType = FileType {
    file_format: &FileFormat {
        id: 5_531_898,
        source_type: SourceType::Wikidata,
        name: "General Exchange Format",
        extensions: &["gxf"],
        media_types: &["application/gxf"],
        signatures: &[],
        related_formats: &[],
    },
};
