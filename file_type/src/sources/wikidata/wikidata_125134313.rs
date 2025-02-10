use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125134313: FileType = FileType {
    file_format: &FileFormat {
        id: 125_134_313,
        source_type: SourceType::Wikidata,
        name: "YAM emailcache",
        extensions: &["emailcache"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
