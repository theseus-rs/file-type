use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27895555: FileType = FileType {
    file_format: &FileFormat {
        id: 27_895_555,
        source_type: SourceType::Wikidata,
        name: "ADX, version 3",
        extensions: &["adx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
