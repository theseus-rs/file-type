use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_76515169: FileType = FileType {
    file_format: &FileFormat {
        id: 76_515_169,
        source_type: SourceType::Wikidata,
        name: "Windows Runtime Metadata",
        extensions: &["winmd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
