use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
