use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110212801: FileType = FileType {
    file_format: &FileFormat {
        id: 110_212_801,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version X5",
        extensions: &["ppb", "ppp", "ppx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
