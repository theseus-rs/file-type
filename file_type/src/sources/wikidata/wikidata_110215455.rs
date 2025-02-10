use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110215455: FileType = FileType {
    file_format: &FileFormat {
        id: 110_215_455,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication, version SE",
        extensions: &["ppb", "ppp", "ppx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
