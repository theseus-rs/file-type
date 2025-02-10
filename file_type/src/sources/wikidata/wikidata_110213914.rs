use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110213914: FileType = FileType {
    file_format: &FileFormat {
        id: 110_213_914,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version X7",
        extensions: &["ppb", "ppp", "ppx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
