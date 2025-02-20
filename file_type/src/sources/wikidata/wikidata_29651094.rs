use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29651094: FileType = FileType {
    file_format: &FileFormat {
        id: 29_651_094,
        source_type: SourceType::Wikidata,
        name: "ULTRA Compressed Archive",
        extensions: &["uca"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
