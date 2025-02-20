use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27826383: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_383,
        source_type: SourceType::Wikidata,
        name: "Reduced resolution dataset external file",
        extensions: &["rde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
