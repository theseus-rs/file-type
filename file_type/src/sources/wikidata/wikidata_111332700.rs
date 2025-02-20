use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111332700: FileType = FileType {
    file_format: &FileFormat {
        id: 111_332_700,
        source_type: SourceType::Wikidata,
        name: "Roland S-5xx series floppy disk image",
        extensions: &["out", "sdk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
