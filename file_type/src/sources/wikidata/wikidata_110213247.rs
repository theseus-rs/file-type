use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110213247: FileType = FileType {
    file_format: &FileFormat {
        id: 110_213_247,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version X6",
        extensions: &["ppb", "ppp", "ppx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
