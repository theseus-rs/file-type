use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132146563: FileType = FileType {
    file_format: &FileFormat {
        id: 132_146_563,
        source_type: SourceType::Wikidata,
        name: "BrailleBlaster XML File",
        extensions: &["bbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
