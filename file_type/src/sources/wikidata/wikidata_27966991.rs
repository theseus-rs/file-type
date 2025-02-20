use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27966991: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_991,
        source_type: SourceType::Wikidata,
        name: "Art & Magic",
        extensions: &["aam"],
        media_types: &["audio/x-mod"],
        signatures: &[],
        related_formats: &[],
    },
};
