use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27923693: FileType = FileType {
    file_format: &FileFormat {
        id: 27_923_693,
        source_type: SourceType::Wikidata,
        name: "DTED Cells Directory File",
        extensions: &["dir"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
