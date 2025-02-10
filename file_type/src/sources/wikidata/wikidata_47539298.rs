use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47539298: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_298,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Block Attribute Template",
        extensions: &["blk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
