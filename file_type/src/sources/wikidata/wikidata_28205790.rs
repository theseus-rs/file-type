use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205790: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_790,
        source_type: SourceType::Wikidata,
        name: "FullPic Picture Format",
        extensions: &["ful"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
