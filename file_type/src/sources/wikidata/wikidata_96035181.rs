use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_96035181: FileType = FileType {
    file_format: &FileFormat {
        id: 96_035_181,
        source_type: SourceType::Wikidata,
        name: "LEDA",
        extensions: &["gw", "lgr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
