use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27907426: FileType = FileType {
    file_format: &FileFormat {
        id: 27_907_426,
        source_type: SourceType::Wikidata,
        name: "Amiga Disk File, compressed",
        extensions: &["adz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
