use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113557107: FileType = FileType {
    file_format: &FileFormat {
        id: 113_557_107,
        source_type: SourceType::Wikidata,
        name: "Virtual CD-ROM, Uncompressed",
        extensions: &["fcd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
