use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110215299: FileType = FileType {
    file_format: &FileFormat {
        id: 110_215_299,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version X9",
        extensions: &["ppb", "ppp", "ppx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
